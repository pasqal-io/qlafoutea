//! Register intermediate language

use std::sync::Arc;

use serde::Serialize;

use crate::types::units::{Coordinates, Micrometers};

#[derive(Clone)]
pub struct Register {
    pub coordinates: Arc<[Coordinates<Micrometers>]>,
}

#[allow(clippy::len_without_is_empty)]
impl Register {
    /// Return the number of qubits in the register.
    pub fn len(&self) -> usize {
        self.coordinates.len()
    }
}

impl Serialize for Register {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema(
            self.coordinates
                .iter()
                .enumerate()
                .map(|(index, c)| AtomSchema {
                    name: format!("{index}"),
                    x: c.x.into_inner(),
                    y: c.y.into_inner(),
                })
                .collect(),
        );
        schema.serialize(serializer)
    }
}

#[derive(Serialize)]
pub struct Schema(Vec<AtomSchema>);

#[derive(Serialize)]
pub struct AtomSchema {
    x: f64,
    y: f64,
    name: String,
}
