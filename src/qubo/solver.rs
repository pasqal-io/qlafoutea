use argmin_math::{ArgminAdd, ArgminMul, ArgminSub};

use crate::register::{CoordinatesUM, Register};

impl ArgminAdd<Self, Self> for Register {
    fn add(&self, other: &Self) -> Self {
        let coordinates = self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| CoordinatesUM {
                x: a.x + b.x,
                y: a.y + b.y,
            })
            .collect();
        Register { coordinates }
    }
}

impl ArgminSub<Self, Self> for Register {
    fn sub(&self, other: &Self) -> Self {
        let coordinates = self
            .coordinates
            .iter()
            .zip(other.coordinates.iter())
            .map(|(a, b)| CoordinatesUM {
                x: a.x - b.x,
                y: a.y - b.y,
            })
            .collect();
        Register { coordinates }
    }
}

impl ArgminMul<f64, Register> for Register {
    fn mul(&self, factor: &f64) -> Register {
        let coordinates = self
            .coordinates
            .iter()
            .map(|a| CoordinatesUM {
                x: factor * a.x,
                y: factor * a.y,
            })
            .collect();
        Register { coordinates }
    }
}
