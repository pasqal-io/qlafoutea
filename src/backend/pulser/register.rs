//! Register intermediate language

use std::sync::Arc;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

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

impl<'de> Deserialize<'de> for Register {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        let coordinates = schema
            .0
            .into_iter()
            .map(|atom| Coordinates {
                x: crate::types::units::Value::new(atom.x),
                y: crate::types::units::Value::new(atom.y),
            })
            .collect_vec();
        let register = Register {
            coordinates: coordinates.into(),
        };
        Ok(register)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Schema(Vec<AtomSchema>);

#[derive(Deserialize, Serialize)]
pub struct AtomSchema {
    x: f64,
    y: f64,
    name: String,
}
