use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{backend, runtime::run::Sample};

pub mod max3sat;

/// Formats understood by the various frontends.
///
/// As of this writing, we read everything from yaml source files.
#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Input {
    /// Raw QUBO constraints.
    #[serde(rename = "qubo")]
    Qubo(backend::qubo::Constraints),

    /// An input for maximal 3SAT.
    #[serde(rename = "max3sat")]
    Max3Sat(max3sat::Input),
}

impl Input {
    /// Compile the input to a set of QUBO constraints.
    pub fn to_constraints(&self) -> Result<backend::qubo::Constraints, anyhow::Error> {
        match *self {
            Self::Max3Sat(ref input) => input.to_qubo().context("failed to resolve QUBO"),
            Self::Qubo(ref input) => Ok(input.clone()),
        }
    }
}

impl Input {
    /// Process results obtained from the QPU or emulator.
    ///
    /// Typically, use the input to turn them into something human-readable.
    pub fn handle_results(&self, samples: &[Sample]) -> Result<(), anyhow::Error> {
        match *self {
            Self::Max3Sat(ref input) => input.handle_results(samples),
            Self::Qubo(_) => {
                let mut writer = csv::Writer::from_writer(std::io::stdout());
                for record in samples {
                    writer.serialize(record)?;
                }
                Ok(())
            }
        }
    }
}
