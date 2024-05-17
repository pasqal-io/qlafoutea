use std::{collections::HashMap, rc::Rc};

use serde::Serialize;

use crate::backend::{
    device::Device,
    pulser::{device::ChannelId, pulse::Pulse, register::Register},
};

pub struct Sequence {
    register: Rc<Register>,
    device: Rc<Device>,
    pulse: Rc<Pulse>,
    channels: Vec<Rc<str>>,
}

impl Sequence {
    pub fn new(device: Device, register: Register, pulse: Pulse, channels: &[Rc<str>]) -> Self {
        Self {
            register: Rc::new(register),
            device: Rc::new(device),
            pulse: Rc::new(pulse),
            channels: channels.to_vec(),
        }
    }
}

impl Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            version: "1",
            name: "qlafoutea compilation target",
            register: self.register.clone(),
            device: self.device.clone(),
            variables: HashMap::new(),
            operations: vec![self.pulse.clone()],
            measurement: None,
            channels: self
                .channels
                .iter()
                .map(|chan| (chan.to_string(), ChannelId(chan.to_string())))
                .collect(),
        };
        schema.serialize(serializer)
    }
}

#[derive(Serialize)]
struct Schema {
    version: &'static str,
    variables: HashMap<String, ()>, // always empty for the time being.
    register: Rc<Register>,
    device: Rc<Device>,
    name: &'static str,
    operations: Vec<Rc<Pulse>>,
    channels: HashMap<String, ChannelId>,
    measurement: Option<()>, // always None for the time being.
}
