use crate::types::units::{Inv, Microseconds, Mul, Pow6, Rad, Value};

#[derive(Clone, Copy, Debug)]
pub struct C6Coeff {
    /// A value in rad/µs x µm^6.
    value_rad_per_us_times_us_64: f64,
}
impl C6Coeff {
    // Minimal index.
    pub const MIN: u32 = 50;

    // Maximal index.
    pub const MAX: u32 = 100;

    /// C_6/hbar (in  rad/µs x µm^6), for Rydberg levels between 50 and 100.
    ///
    /// The values were calculated using ARC_ and double checked with
    /// PairInteraction_.
    ///
    /// _ARC: https://arc-alkali-rydberg-calculator.readthedocs.io/
    /// _PairInteraction: https://www.pairinteraction.org/
    pub fn new(level: u32) -> Option<Self> {
        if level < Self::MIN {
            return None;
        }
        if level > Self::MAX {
            return None;
        }
        let value = match level {
            50 => 96120.72,
            51 => 122241.6,
            52 => 154693.02,
            53 => 194740.36,
            54 => 243973.91,
            55 => 304495.01,
            56 => 378305.98,
            57 => 468027.05,
            58 => 576714.85,
            59 => 707911.38,
            60 => 865723.02,
            61 => 1054903.11,
            62 => 1281042.11,
            63 => 1550531.15,
            64 => 1870621.31,
            65 => 2249728.57,
            66 => 2697498.69,
            67 => 3224987.51,
            68 => 3844734.37,
            69 => 4571053.32,
            70 => 5420158.53,
            71 => 6410399.4,
            72 => 7562637.31,
            73 => 8900342.14,
            74 => 10449989.62,
            75 => 12241414.53,
            76 => 14308028.03,
            77 => 16687329.94,
            78 => 19421333.62,
            79 => 22557029.94,
            80 => 26146720.74,
            81 => 30248886.65,
            82 => 34928448.69,
            83 => 40257623.67,
            84 => 46316557.88,
            85 => 53194043.52,
            86 => 60988354.64,
            87 => 69808179.15,
            88 => 79773468.88,
            89 => 91016513.07,
            90 => 103677784.57,
            91 => 117933293.96,
            92 => 133943541.9,
            93 => 151907135.94,
            94 => 172036137.34,
            95 => 194562889.89,
            96 => 219741590.56,
            97 => 247850178.91,
            98 => 279192193.77,
            99 => 314098829.39,
            100 => 352931119.11,
            _ => return None,
        };
        Some(Self {
            value_rad_per_us_times_us_64: value,
        })
    }

    pub fn value_rad_per_us_times_um_6(&self) -> f64 {
        self.value_rad_per_us_times_us_64
    }

    pub fn value_rad_per_us_times(&self) -> Value<Mul<Rad, Inv<Pow6<Microseconds>>>> {
        Value::new(self.value_rad_per_us_times_us_64)
    }
}
