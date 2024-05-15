//! QUBO format as an intermediate language
//!
//! QUBO (Quadratic unconstrained binary optimization) is an optimization problem
//! that generally maps well to quantum registers.
//!
//! Given a set of weights Q (a matrix), solving the QUBO problem means finding
//! the best values x in {0, 1} ^ n to minimize the following formula:
//!
//! sum_{i, j}(Q[i, j] * x [i] * x[j])

use argmin::{
    core::{CostFunction, Executor},
    solver::neldermead::NelderMead,
};
use itertools::Itertools;
use medians::Medianf64;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Deserialize;

use crate::{
    device::Device, pulser::register::Register, types::units::Coordinates,
    types::units::Micrometers, types::Quality,
};

pub mod format;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("index out of bounds")]
    IndexOutOfBounds {
        x: usize,
        y: usize,
        num_nodes: usize,
    },

    #[error("layout error #?")]
    Layout(argmin::core::Error),

    #[error("failed to layout")]
    NoSolution,
}

pub struct Options {
    pub seed: u64,
    pub min_quality: Quality,
    pub max_iters: u64,
}

/// A set of qubo constraints.
///
/// For (de)serialization, please use `format::Format`.
#[derive(Debug, Deserialize)]
pub struct Constraints {
    // FIXME: Check that there is no NaN, no infinite, that all values are reasonable.
    /// A symmetric matrix of weights of size `num_nodes`.
    ///
    /// In practice, the sub-diagonal part of the matrix is redundant.
    /// We could save ~half of the memory by removing this part, but at
    /// the expense of increasing computation costs.
    ///
    /// In this implementation, we don't make the effort.
    data: Vec<f64>,
    num_nodes: usize,
}
#[allow(clippy::len_without_is_empty)]
impl Constraints {
    #[allow(dead_code)]
    pub fn try_new(num_nodes: usize, data: Vec<f64>) -> Option<Self> {
        if data.len() != num_nodes * num_nodes {
            return None;
        }
        Some(Self { data, num_nodes })
    }

    /// Return the number of constraints.
    pub fn len(&self) -> usize {
        self.num_nodes * self.num_nodes / 2
    }

    pub fn layout(&self, device: &Device, options: &Options) -> Option<(Register, Quality)> {
        (options.seed..std::u64::MAX)
            .into_par_iter()
            .find_map_any(|seed| {
                let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

                // Set initial search points.
                //
                // Our search space has 2 * num_node dimensions (we're looking for 2 coordinates per node).
                // By definition, Nelder-Mead must take dimensions + 1 starting points.
                //
                // Since we wish to be reproducible, we initialize these points from `rng`, which can be
                // seeded by the caller.
                let mut params = Vec::with_capacity(self.num_nodes * 2 + 1);
                for _ in 0..self.num_nodes {
                    let mut state = vec![0f64; self.num_nodes * 2];
                    rng.fill(state.as_mut_slice());
                    params.push(state);
                }
                let solver = NelderMead::new(params);

                let cost = Cost {
                    constraints: self,
                    device,
                };

                let optimized = Executor::new(cost, solver)
                    .configure(|state| state.max_iters(options.max_iters).target_cost(1e-6))
                    .run()
                    .expect("Error in the execution of register optimizer");
                let quality = 1. - optimized.state.best_cost.atan() / std::f64::consts::FRAC_PI_2;
                let quality = Quality::new(quality);
                let coordinates = match optimized.state.best_param {
                    None => return None,
                    Some(v) => {
                        assert!(v.len() % 2 == 0);
                        let mut iter = v.into_iter();
                        let mut coordinates = Vec::with_capacity(self.num_nodes);
                        while let Some((x, y)) = iter.next_tuple() {
                            coordinates.push(Coordinates::<Micrometers>::new(x, y))
                        }
                        coordinates
                    }
                };
                let register = Register {
                    coordinates: coordinates.into(),
                };

                if quality >= options.min_quality {
                    eprintln!("succeeded with seed {seed}");
                    Some((register, quality))
                } else {
                    None
                }
            })
    }

    pub fn omega(&self) -> f64 {
        self.data
            .iter()
            .cloned()
            .filter(|f| *f > 0.)
            .collect_vec()
            .medf_unchecked()
    }
}

impl Constraints {
    pub fn at(&self, x: usize, y: usize) -> Result<f64, Error> {
        let index = self.index(x, y)?;
        Ok(self.data[index])
    }

    fn index(&self, x: usize, y: usize) -> Result<usize, Error> {
        let (x, y) = if x >= self.num_nodes || y >= self.num_nodes {
            return Err(Error::IndexOutOfBounds {
                x,
                y,
                num_nodes: self.num_nodes,
            });
        } else if x <= y {
            (x, y)
        } else {
            (y, x)
        };
        Ok(self.num_nodes * y + x)
    }
}

struct Cost<'a> {
    constraints: &'a Constraints,
    device: &'a Device,
}
impl<'a> Cost<'a> {
    fn actual_interaction(
        &self,
        first: Coordinates<Micrometers>,
        second: Coordinates<Micrometers>,
    ) -> f64 {
        self.device
            .interaction_coeff()
            .value_rad_per_us_times_us_64()
            / first.sqdist(&second).powi(3)
    }
    fn expected_interaction(&self, x: usize, y: usize) -> f64 {
        self.constraints.at(x, y).unwrap()
    }
}

impl<'a> CostFunction for Cost<'a> {
    type Param = Vec<f64>;

    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, anyhow::Error> {
        debug_assert_eq!(param.len(), 2 * self.constraints.num_nodes);
        let mut total = 0.;

        for i in 0..self.constraints.num_nodes {
            let first = Coordinates::<Micrometers>::new(param[2 * i], param[2 * i + 1]);
            for j in i + 1..self.constraints.num_nodes {
                let second = Coordinates::<Micrometers>::new(param[2 * j], param[2 * j + 1]);
                let actual_interaction = self.actual_interaction(first, second);
                let expected_interaction = self.expected_interaction(i, j);
                let diff = (actual_interaction - expected_interaction).powi(2);
                if i == j {
                    total += diff
                } else {
                    total += 2. * diff
                }
            }
        }
        let result = total.sqrt();
        Ok(result)
    }
}

#[test]
// Test that the cost is roughly the same to the one we compute in Python.
fn test_cost_function_vs_python() {
    let evaluator = Cost {
        device: &Device::analog(),
        constraints: &Constraints::try_new(
            5,
            vec![
                -10.0,
                19.7365809,
                19.7365809,
                5.42015853,
                5.42015853,
                19.7365809,
                -10.0,
                20.67626392,
                0.17675796,
                0.85604541,
                19.7365809,
                20.67626392,
                -10.0,
                0.85604541,
                0.17675796,
                5.42015853,
                0.17675796,
                0.85604541,
                -10.0,
                0.32306662,
                5.42015853,
                0.85604541,
                0.17675796,
                0.32306662,
                -10.0,
            ],
        )
        .unwrap(),
    };
    let param = vec![
        0.5488135, 0.71518937, 0.60276338, 0.54488318, 0.4236548, 0.64589411, 0.43758721, 0.891773,
        0.96366276, 0.38344152,
    ];
    let cost = evaluator.cost(&param).unwrap();
    let python_cost = 149417981060.90808; // Achieved manually from running Python implementation.
    let diff = f64::abs(cost - python_cost) / f64::max(cost, python_cost);
    assert!(
        diff <= 0.01,
        "Expected < 1% difference between {} and {}, got {}",
        cost,
        python_cost,
        diff
    );
}
