//! QUBO format as an intermediate language
//!
//! QUBO (Quadratic unconstrained binary optimization) is an optimization problem
//! that generally maps well to quantum registers.
//!
//! Given a set of weights Q (a matrix), solving the QUBO problem means finding
//! the best values x in {0, 1} ^ n to minimize the following formula:
//!
//! sum_{i, j}(Q[i, j] * x [i] * x[j])

use std::{fmt::Display, sync::Arc};

use argmin::{
    core::{CostFunction, Executor},
    solver::neldermead::NelderMead,
};
use itertools::Itertools;
use medians::Medianf64;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    backend::{device::Device, pulser::register::Register},
    types::{
        units::{self, Coordinates, Inv, Micrometers, Microseconds, Mul, Rad},
        Quality,
    },
};

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

    #[error("algorithm cannot layout off-diagonal negative values")]
    NegativeValueOffDiagonal,

    #[error("algorithm cannot layout on-diagonal positive values")]
    PositiveValueOnDiagonal,

    #[error("value is infinite or not a number")]
    InfiniteValue,
}

#[derive(Clone, Debug)]
pub struct Options {
    pub seed: u64,
    pub min_quality: Quality,
    pub max_iters: u64,
    pub overflow_protection_threshold: f64,
    pub overflow_protection_factor: f64,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            seed: 0,
            min_quality: Quality::new(0.2),
            max_iters: 4_000,
            overflow_protection_threshold: 0.9,
            overflow_protection_factor: 1_000.,
        }
    }
}

/// A set of qubo constraints.
///
/// For (de)serialization, please use `format::Format`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Constraints {
    /// A symmetric matrix of weights of size `num_nodes`.
    ///
    /// In practice, the sub-diagonal part of the matrix is redundant.
    /// We could save ~half of the memory by removing this part, but at
    /// the expense of increasing computation costs.
    ///
    /// In this implementation, we don't make the effort.
    data: Vec<f64>,
    num_nodes: usize,
    names: Vec<Arc<str>>,
}

#[allow(clippy::len_without_is_empty)]
impl Constraints {
    pub fn new(num_nodes: usize, names: Vec<Arc<str>>) -> Self {
        let data = vec![0.; num_nodes * num_nodes];
        assert_eq!(names.len(), num_nodes);
        Self {
            data,
            num_nodes,
            names,
        }
    }
    pub fn from_const<const N: usize>(data: [[f64; N]; N], names: Vec<Arc<str>>) -> Self {
        assert_eq!(names.len(), data.len());
        let data = data.iter().flat_map(|d| d.iter()).cloned().collect();
        Self {
            data,
            num_nodes: N,
            names,
        }
    }
    #[allow(dead_code)]
    pub fn try_new(num_nodes: usize, data: Vec<f64>, names: Vec<Arc<str>>) -> Option<Self> {
        if data.len() != num_nodes * num_nodes {
            return None;
        }
        if names.len() != num_nodes {
            return None;
        }
        Some(Self {
            data,
            num_nodes,
            names,
        })
    }

    /// Return the number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.num_nodes * self.num_nodes / 2
    }

    /// Return the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    pub fn check_compilable_subset(&self) -> Result<(), Error> {
        for (index, x) in self.data.iter().enumerate() {
            if !x.is_finite() {
                return Err(Error::InfiniteValue);
            }
            let is_diagonal = index % self.num_nodes == index / self.num_nodes;
            if is_diagonal {
                if x.is_sign_positive() {
                    return Err(Error::PositiveValueOnDiagonal);
                }
            } else if x.is_sign_negative() {
                return Err(Error::NegativeValueOffDiagonal);
            }
        }
        Ok(())
    }

    /// Attempt to layout a set of constraints as a Register.
    ///
    /// In the current implementation, we run N concurrent instances of a Nelder-Mead optimizer,
    /// with distinct start states, where N is determined from the number of cores on the computer.
    ///
    /// In case of success, returns:
    /// - Register: the geometry;
    /// - Quality: an abstract measure of quality, where 0 is really bad and 1 is optimal;
    /// - seed: the seed with which we found a solution.
    pub fn layout(&self, device: &Device, options: &Options) -> Option<(Register, Quality, u64)> {
        // self.check_compilable_subset().expect("invalid content");
        // FIXME: We should add laser channels to the parameters we optimize!

        (0..u64::MAX).into_par_iter().find_map_any(|seed| {
            let seed = seed.wrapping_add(options.seed);
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
                options: options.clone(),
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
                        let name = self.names[coordinates.len()].clone();
                        coordinates.push((Coordinates::<Micrometers>::new(x, y), name))
                    }
                    coordinates
                }
            };
            let register = Register {
                coordinates: coordinates.into(),
            };

            if quality >= options.min_quality {
                Some((register, quality, seed))
            } else {
                eprintln!("...testing seed {seed} => insufficient quality {}", quality);
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

    fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut f64, Error> {
        let index = self.index(x, y)?;
        Ok(&mut self.data[index])
    }

    /// Change a value at given coordinates.
    pub fn delta_at(&mut self, x: usize, y: usize, delta: f64) -> Result<(), Error> {
        let ref_mut = self.get_mut(x, y)?;
        *ref_mut += delta;
        Ok(())
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

impl Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = "[".to_string();
        for y in 0..self.num_nodes {
            let mut line = if y == 0 {
                "[".to_string()
            } else {
                " [".to_string()
            };
            for x in 0..self.num_nodes {
                if x == 0 {
                    line.push_str(&format!("{}", self.at(x, y).unwrap()));
                } else {
                    line.push_str(&format!(", {}", self.at(x, y).unwrap()));
                }
            }
            buf.push_str(&format!(
                "{}]{}",
                line,
                if y + 1 == self.num_nodes { "]" } else { ",\n" }
            ));
        }
        f.collect_str(&buf)
    }
}

struct Cost<'a> {
    constraints: &'a Constraints,
    device: &'a Device,
    options: Options,
}
impl Cost<'_> {
    fn actual_interaction(
        &self,
        first: Coordinates<Micrometers>,
        second: Coordinates<Micrometers>,
    ) -> units::Value<Mul<Rad, Inv<Microseconds>>> {
        units::Value::new(
            self.device
                .interaction_coeff()
                .value_rad_per_us_times_um_6()
                / first.sqdist(&second).cube().into_inner(),
        )
    }
    fn expected_interaction(
        &self,
        x: usize,
        y: usize,
    ) -> units::Value<Mul<Rad, Inv<Microseconds>>> {
        units::Value::new(self.constraints.at(x, y).unwrap())
    }
}

impl CostFunction for Cost<'_> {
    type Param = Vec<f64>;

    type Output = f64;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, anyhow::Error> {
        debug_assert_eq!(param.len(), 2 * self.constraints.num_nodes);
        use crate::types::units::*;

        // First component: distance to our objective.
        let distance = {
            let mut total: Value<Square<Mul<Rad, Inv<Microseconds>>>> = Value::new(0.);
            for i in 0..self.constraints.num_nodes {
                let first = Coordinates::<Micrometers>::new(param[2 * i], param[2 * i + 1]);
                for j in i + 1..self.constraints.num_nodes {
                    let second = Coordinates::<Micrometers>::new(param[2 * j], param[2 * j + 1]);
                    let actual_interaction = self.actual_interaction(first, second);
                    let expected_interaction = self.expected_interaction(i, j);
                    let diff = (actual_interaction - expected_interaction).sq();
                    if i == j {
                        total += diff
                    } else {
                        total += 2. * diff
                    }
                }
            }
            total.sqrt()
        };

        // Second component: make sure that all the atoms fit on the device
        // (aka "overflow protection")
        let max_sq_distance_to_center = param
            .iter()
            .tuples()
            .map(|(x, y)| x * x + y * y)
            .reduce(f64::max)
            .unwrap();
        let overflow_risk = max_sq_distance_to_center / self.device.max_sq_distance_to_center();
        let overflow_cost = if overflow_risk < self.options.overflow_protection_threshold {
            0.
        } else {
            self.options.overflow_protection_factor
                * (overflow_risk - self.options.overflow_protection_threshold).exp()
        };

        Ok(overflow_cost + distance.into_inner())
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
            vec!["a", "b", "c", "d", "e"]
                .into_iter()
                .map(|x| x.to_string().into())
                .collect_vec(),
        )
        .unwrap(),
        options: Options::default(),
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
