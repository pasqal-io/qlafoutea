use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

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
    pub fn register(&self) -> &Register {
        &self.register
    }
    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn pulse(&self) -> &Pulse {
        &self.pulse
    }
    pub fn channels(&self) -> &[Rc<str>] {
        &self.channels
    }
}

impl<'de> Deserialize<'de> for Sequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        match schema.operations.len() {
            0 => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Other("empty operations"),
                    &"exactly one operation",
                ))
            }
            n if n > 1 => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Other(&format!("{} operations", n)),
                    &"exactly one operation",
                ))
            }
            _ => {}
        }
        Ok(Self {
            register: schema.register,
            device: schema.device,
            pulse: schema.operations[0].clone(),
            channels: schema.channels.keys().map(|k| k.clone().into()).collect(),
        })
    }
}

impl Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            version: "1".to_string(),
            name: "qlafoutea compilation target".to_string(),
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

#[derive(Deserialize, Serialize)]
struct Schema {
    version: String,
    variables: HashMap<String, ()>, // always empty for the time being.
    register: Rc<Register>,
    device: Rc<Device>,
    name: String,
    operations: Vec<Rc<Pulse>>,
    channels: HashMap<String, ChannelId>,
    measurement: Option<()>, // always None for the time being.
}
