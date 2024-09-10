use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Sub},
};

use argmin_math::{ArgminAdd, ArgminSub};
use serde::{Deserialize, Serialize};

pub trait Unit: Copy {}

// A marker for a value in micrometers.
#[derive(Clone, Copy)]
pub struct Micrometers;
impl Unit for Micrometers {}

// A marker for a value in microseconds.
#[derive(Clone, Copy)]
pub struct Microseconds;
impl Unit for Microseconds {}

// A marker for a value in micrometers.
#[derive(Clone, Copy)]
pub struct Rad;
impl Unit for Rad {}

#[derive(Clone, Copy)]
pub struct Mul<T, U>(PhantomData<(T, U)>);
impl<T: Unit, U: Unit> Unit for Mul<T, U> {}

pub type Square<T> = Mul<T, T>;
pub type Cube<T> = Mul<T, Square<T>>;
pub type Pow6<T> = Mul<Cube<T>, Cube<T>>;

#[derive(Clone, Copy)]
pub struct Inv<T>(PhantomData<T>);
impl<T: Unit> Unit for Inv<T> {}

#[derive(Clone, Copy)]
pub struct Value<T>
where
    T: Unit,
{
    value: f64,
    _phantom: PhantomData<T>,
}
impl<T> Value<T>
where
    T: Unit,
{
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
    pub fn into_inner(self) -> f64 {
        self.value
    }
    pub fn sq(self) -> Value<Mul<T, T>> {
        Value {
            value: self.value * self.value,
            _phantom: PhantomData,
        }
    }
    pub fn cube(self) -> Value<Mul<T, Mul<T, T>>> {
        Value {
            value: self.value * self.value * self.value,
            _phantom: PhantomData,
        }
    }
}
impl<T> Value<Square<T>>
where
    T: Unit,
{
    pub fn sqrt(self) -> Value<T> {
        Value {
            value: self.value.sqrt(),
            _phantom: PhantomData,
        }
    }
}
impl<'de, T> Deserialize<'de> for Value<T>
where
    T: Unit,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::new(f64::deserialize(deserializer)?))
    }
}
impl<T> Serialize for Value<T>
where
    T: Unit,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

/// Coordinates in micrometers.
#[derive(Clone, Copy)]
pub struct Coordinates<T>
where
    T: Unit,
{
    pub x: Value<T>,
    pub y: Value<T>,
}

impl<T> Coordinates<T>
where
    T: Unit,
{
    pub fn new(x: f64, y: f64) -> Self {
        Coordinates {
            x: Value::new(x),
            y: Value::new(y),
        }
    }
    pub fn sqdist(&self, other: &Coordinates<T>) -> Value<Mul<T, T>> {
        (other.x - self.x).sq() + (other.y - self.y).sq()
    }
}

impl<T> ArgminAdd<Self, Self> for Value<T>
where
    T: Unit,
{
    fn add(&self, other: &Self) -> Self {
        Self {
            value: self.value + other.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> ArgminSub<Self, Self> for Value<T>
where
    T: Unit,
{
    fn sub(&self, other: &Self) -> Self {
        Self {
            value: self.value - other.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> Add<Self> for Value<T>
where
    T: Unit,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> AddAssign<Self> for Value<T>
where
    T: Unit,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl<T> std::ops::Mul<Value<T>> for f64
where
    T: Unit,
{
    type Output = Value<T>;

    fn mul(self, rhs: Value<T>) -> Self::Output {
        Value {
            value: self * rhs.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> Sub<Self> for Value<T>
where
    T: Unit,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> ArgminAdd<Self, Self> for Coordinates<T>
where
    T: Unit,
{
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ArgminSub<Self, Self> for Coordinates<T>
where
    T: Unit,
{
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/*
impl<T> ArgminMul<f64, Coordinates<T>> for Coordinates<T> {
    fn mul(&self, factor: &f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}
*/
