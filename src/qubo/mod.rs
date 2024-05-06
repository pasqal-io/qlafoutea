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
use serde::{Deserialize, Serialize};

use crate::{
    device::Device,
    register::{CoordinatesUM, Register},
    types::Quality,
};

mod format;
mod solver;

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

/// A set of qubo constraints.
///
/// For (de)serialization, please use `format::Format`.
#[derive(Debug, Deserialize, Serialize)]
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
impl Constraints {
    /// Initialize for `size` nodes.
    pub fn with_size(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size * size),
            num_nodes: size,
        }
    }

    /// Return the number of constraints.
    pub fn len(&self) -> usize {
        self.num_nodes * self.num_nodes / 2
    }

    /// Add a constraint between two nodes.
    ///
    /// Note that constraints are symmetric - we always renormalize so that x <= y.
    pub fn add_constraint(&mut self, x: usize, y: usize, value: f64) -> Result<(), Error> {
        let cell = self.at_mut(x, y)?;
        *cell = value;
        Ok(())
    }

    pub fn layout<Rng: rand::Rng>(
        &self,
        device: &Device,
        mut rng: Rng,
    ) -> Result<(Register, Quality), Error> {
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
            .configure(|state| state.max_iters(100))
            .run()
            .map_err(Error::Layout)?;
        let coordinates = match optimized.state.best_param {
            None => return Err(Error::NoSolution),
            Some(v) => {
                assert!(v.len() % 2 == 0);
                let mut iter = v.into_iter();
                let mut coordinates = Vec::with_capacity(self.num_nodes);
                while let Some((x, y)) = iter.next_tuple() {
                    coordinates.push(CoordinatesUM { x, y })
                }
                coordinates
            }
        };
        let register = Register { coordinates };
        // Normalize distance into a value in [0, 1[
        let quality = optimized.state.best_cost.atan() / std::f64::consts::FRAC_PI_2;

        Ok((register, Quality::new(quality)))
    }

    pub fn median(&self) -> f64 {
        self.data.medf_unchecked()
    }
}

impl Constraints {
    pub fn at(&self, x: usize, y: usize) -> Result<f64, Error> {
        let index = self.index(x, y)?;
        Ok(self.data[index])
    }
    fn at_mut(&mut self, x: usize, y: usize) -> Result<&mut f64, Error> {
        let index = self.index(x, y)?;
        Ok(&mut self.data[index])
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
    fn actual_interaction(&self, first: CoordinatesUM, second: CoordinatesUM) -> f64 {
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
            let first = CoordinatesUM {
                x: param[2 * i],
                y: param[2 * i + 1],
            };
            for j in i + 1..self.constraints.num_nodes {
                let second = CoordinatesUM {
                    x: param[2 * j],
                    y: param[2 * j + 1],
                };
                let actual_interaction = self.actual_interaction(first, second);
                let expected_interaction = self.expected_interaction(i, j);
                total += (actual_interaction - expected_interaction).powi(2);
            }
        }
        Ok(total.sqrt())
    }
}
