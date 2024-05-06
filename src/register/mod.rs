//! Register intermediate language

use argmin_math::{ArgminAdd, ArgminMul, ArgminSub};
use serde::{Deserialize, Serialize};

use crate::pulser::register_schema::{Atom, Register as RegisterAR};

#[derive(Clone, Copy)]
pub struct CoordinatesUM {
    pub x: f64,
    pub y: f64,
}

impl CoordinatesUM {
    pub fn sqdist(&self, other: &CoordinatesUM) -> f64 {
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2)
    }
}

impl ArgminAdd<Self, Self> for CoordinatesUM {
    fn add(&self, other: &Self) -> Self {
        CoordinatesUM {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ArgminSub<Self, Self> for CoordinatesUM {
    fn sub(&self, other: &Self) -> Self {
        CoordinatesUM {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ArgminMul<f64, CoordinatesUM> for CoordinatesUM {
    fn mul(&self, factor: &f64) -> CoordinatesUM {
        CoordinatesUM {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

#[derive(Clone)]
pub struct Register {
    pub coordinates: Vec<CoordinatesUM>,
}

impl Register {
    /// Return the number of qubits in the register.
    pub fn len(&self) -> usize {
        self.coordinates.len()
    }

    pub fn as_abstract_repr(&self) -> RegisterAR {
        RegisterAR {
            layout: None,
            register: self
                .coordinates
                .iter()
                .enumerate()
                .map(|(index, c)| Atom {
                    name: format!("qubit{index}").into(),
                    x: c.x,
                    y: c.y,
                })
                .collect(),
        }
    }
}

impl Serialize for Register {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert to Pulser format.
        let register = self.as_abstract_repr();
        register.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Register {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let register = RegisterAR::deserialize(deserializer)?;
        let coordinates = register
            .register
            .into_iter()
            .map(|atom| CoordinatesUM {
                x: atom.x,
                y: atom.y,
            })
            .collect();
        Ok(Register { coordinates })
    }
}
