pub mod units;

use std::fmt::Display;

/// A quality level in [0, 1]
#[derive(derive_more::Into)]
pub struct Quality(f64);

impl Quality {
    pub const WORST: Quality = Quality(0.);
    pub const BEST: Quality = Quality(1.);
    pub fn new(value: f64) -> Self {
        assert!((0. ..=1.).contains(&value));
        Quality(value)
    }
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0}%", self.0 * 100.)
    }
}
