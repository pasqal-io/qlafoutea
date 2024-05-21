use serde::{Deserialize, Serialize};

use crate::{backend, runtime::run::Sample};

pub mod max3sat;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Input {
    #[serde(rename = "qubo")]
    Qubo(backend::qubo::Constraints),

    #[serde(rename = "max3sat")]
    Max3Sat(max3sat::Input),
}

impl Input {
    pub fn to_constraints(&self) -> Result<backend::qubo::Constraints, anyhow::Error> {
        match *self {
            Self::Max3Sat(ref input) => Ok(input.to_qubo()),
            Self::Qubo(ref input) => Ok(input.clone()),
        }
    }
}

impl Input {
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
