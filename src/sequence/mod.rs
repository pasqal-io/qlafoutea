use std::{collections::HashMap, sync::Arc};

use crate::{
    device::Device,
    pulse::Pulse,
    pulser::sequence_schema::{self, ChannelId, Operation, PulserSequence},
    register::Register,
};

pub struct Sequence {
    register: Register,
    device: Device,
    pulse: Pulse,
    channels: Vec<Arc<str>>,
}

impl Sequence {
    pub fn new(device: Device, register: Register, pulse: Pulse, channels: &[Arc<str>]) -> Self {
        Self {
            register,
            device,
            pulse,
            channels: channels.to_vec(),
        }
    }
    pub fn as_abstract_repr(&self) -> PulserSequence {
        use sequence_schema::PulserSequenceSubtype0 as Seq;
        use sequence_schema::PulserSequenceSubtype0Device as Dev;
        PulserSequence {
            subtype_0: Some(Seq {
                version: "1".to_string(),
                layout: None,
                magnetic_field: None,
                measurement: None, // FIXME: That looks odd, but seems confirmed.
                schema: None,      // FIXME: Check.
                slm_mask_targets: vec![], // FIXME: Check.
                variables: HashMap::new(),
                register: self.register.as_abstract_repr().register,
                device: Dev::Device(Box::new(self.device.as_abstract_repr())),
                name: "qlafoutis compilation target".to_string(),
                operations: vec![Operation::Pulse(self.pulse.as_abstract_repr())],
                channels: self
                    .channels
                    .iter()
                    .map(|chan| (chan.to_string(), ChannelId(chan.to_string())))
                    .collect(),
            }),
            subtype_1: None,
        }
    }
}
