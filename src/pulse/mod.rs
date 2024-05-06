use std::sync::Arc;

use crate::pulser::sequence_schema::{
    self, ChannelName, InterpolatedWaveform, ParametrizedNum, ParametrizedNumArray,
};

pub enum Waveform {
    Interpolated {
        values: Vec<f64>,
        timestamps: Vec<f64>,
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
            values: values.to_vec(),
            timestamps,
        }
    }

    pub fn as_abstract_repr(&self) -> sequence_schema::Waveform {
        let Waveform::Interpolated {
            ref values,
            ref timestamps,
        } = *self;
        let duration = timestamps.last().unwrap();
        InterpolatedWaveform {
            duration: ParametrizedNum {
                subtype_0: Some(*duration),
                subtype_1: None,
            },
            kind: "interpolated".to_string(),
            times: ParametrizedNumArray {
                subtype_0: Some(timestamps.clone()),
                subtype_1: None,
                subtype_2: None,
            },
            values: ParametrizedNumArray {
                subtype_0: Some(values.clone()),
                subtype_1: None,
                subtype_2: None,
            },
        }
        .into()
    }
}

pub struct Pulse {
    channel: Arc<str>,
    amplitude: Waveform,
    detuning: Waveform,
}
impl Pulse {
    pub fn new(channel: Arc<str>, amplitude: Waveform, detuning: Waveform) -> Self {
        Self {
            channel,
            amplitude,
            detuning,
        }
    }
    pub fn as_abstract_repr(&self) -> sequence_schema::OpPulse {
        sequence_schema::OpPulse {
            amplitude: self.amplitude.as_abstract_repr(),
            detuning: self.detuning.as_abstract_repr(),
            channel: ChannelName(self.channel.to_string()),
            op: "pulse".to_string(),
            phase: ParametrizedNum {
                subtype_0: Some(0.),
                subtype_1: None,
            },
            post_phase_shift: ParametrizedNum {
                subtype_0: Some(0.),
                subtype_1: None,
            },
            protocol: sequence_schema::OpPulseProtocol::MinDelay,
        }
    }
}
