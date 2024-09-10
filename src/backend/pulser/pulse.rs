use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::backend::pulser::waveform::Waveform;

pub struct Pulse {
    channel: Rc<str>,
    amplitude: Waveform,
    detuning: Waveform,
}
impl Pulse {
    pub fn new(channel: Rc<str>, amplitude: Waveform, detuning: Waveform) -> Self {
        Self {
            channel,
            amplitude,
            detuning,
        }
    }
}

impl Serialize for Pulse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            amplitude: self.amplitude.clone(),
            detuning: self.detuning.clone(),
            channel: self.channel.clone(),
            op: "pulse".to_string(),
            phase: 0.,
            post_phase_shift: 0.,
            protocol: "min-delay".to_string(),
        };
        schema.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Pulse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        Ok(Self {
            amplitude: schema.amplitude,
            detuning: schema.detuning,
            channel: schema.channel,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct Schema {
    amplitude: Waveform,
    detuning: Waveform,
    channel: Rc<str>,
    op: String,
    phase: f64,
    post_phase_shift: f64,
    protocol: String,
}
