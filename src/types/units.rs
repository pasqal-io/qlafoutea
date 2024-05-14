use std::marker::PhantomData;

use argmin_math::{ArgminAdd, ArgminMul, ArgminSub};

// A marker for a value in micrometers.
#[derive(Clone, Copy)]
pub struct Micrometers;

/// Coordinates in micrometers.
#[derive(Clone, Copy)]
pub struct Coordinates<T> {
    pub x: f64,
    pub y: f64,
    _phantom: PhantomData<T>,
}

impl<T> Coordinates<T> {
    pub fn new(x: f64, y: f64) -> Self {
        Coordinates {
            x,
            y,
            _phantom: PhantomData,
        }
    }
    pub fn sqdist(&self, other: &Coordinates<T>) -> f64 {
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2)
    }
}

impl<T> ArgminAdd<Self, Self> for Coordinates<T> {
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            _phantom: PhantomData,
        }
    }
}

impl<T> ArgminSub<Self, Self> for Coordinates<T> {
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            _phantom: PhantomData,
        }
    }
}

impl<T> ArgminMul<f64, Coordinates<T>> for Coordinates<T> {
    fn mul(&self, factor: &f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            _phantom: PhantomData,
        }
    }
}
