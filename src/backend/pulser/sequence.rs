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
/*
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
            channels: schema.channels.into_iter().map(|(k, _)| k.into()).collect(),
        })
    }
}
*/
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
