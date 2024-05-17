use std::rc::Rc;

use serde::Serialize;

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
            op: "pulse",
            phase: 0.,
            post_phase_shift: 0.,
            protocol: "min-delay",
        };
        schema.serialize(serializer)
    }
}

#[derive(Serialize)]
struct Schema {
    amplitude: Waveform,
    detuning: Waveform,
    channel: Rc<str>,
    op: &'static str,
    phase: f64,
    post_phase_shift: f64,
    protocol: &'static str,
}
