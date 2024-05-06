use serde::{Deserialize, Serialize};

use super::Constraints;

/// The (de)serialization format for qubo constraints.
#[derive(Deserialize, Serialize)]
#[serde(tag="version")]
pub enum Format {
    /// Version one of the serialization format.
    #[serde(rename="0.1.0")]
    V1(Constraints),
}