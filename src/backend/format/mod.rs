use serde::{Deserialize, Serialize};

use super::pulser::sequence;

#[derive(Deserialize, Serialize)]
pub struct Code {
    pub problem: crate::frontend::Input,
    pub sequence: String,
}
impl Code {
    pub fn try_new(
        problem: crate::frontend::Input,
        sequence: sequence::Sequence,
    ) -> Result<Self, anyhow::Error> {
        let sequence = serde_json::to_string_pretty(&sequence)?;
        Ok(Self { problem, sequence })
    }
}
