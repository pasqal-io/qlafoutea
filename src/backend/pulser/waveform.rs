use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum Waveform {
    Interpolated {
        values: Rc<[f64]>,
        timestamps: Rc<[f64]>,
    },
}

impl Waveform {
    pub fn interpolated(total_duration_ns: f64, values: &[f64]) -> Self {
        assert!(total_duration_ns > 0.);
        assert!(values.len() >= 2);
        let mut timestamps = Vec::with_capacity(values.len());
        timestamps.push(0.);
        for i in 1..values.len() {
            timestamps.push(i as f64 * total_duration_ns / (values.len() as f64 - 1f64));
        }
        assert_eq!(timestamps.len(), values.len());
        Waveform::Interpolated {
            values: values.to_vec().into(),
            timestamps: timestamps.into(),
        }
    }
}

impl Serialize for Waveform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Waveform::Interpolated {
            ref values,
            ref timestamps,
        } = *self;
        let duration = timestamps.last().cloned().unwrap();
        let schema = Schema {
            kind: "interpolated".to_string(),
            duration,
            times: timestamps.iter().map(|v| v / duration).collect(),
            values: values.clone(),
        };
        schema.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Waveform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        Ok(Waveform::Interpolated {
            values: schema.values,
            timestamps: schema.times.iter().map(|v| v * schema.duration).collect(),
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Schema {
    kind: String,
    duration: f64,
    times: Rc<[f64]>,
    values: Rc<[f64]>,
}
