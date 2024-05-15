use std::rc::Rc;

use crate::{
    device::Device,
    pulser::{pulse::Pulse, register::Register, sequence::Sequence, waveform::Waveform},
    qubo::Constraints,
};

pub struct Options {
    pub half_duration_ns: f64,
}

pub fn compile(
    constraints: &Constraints,
    device: Device,
    register: Register,
    options: &Options,
) -> Sequence {
    let omega = constraints.omega();
    let delta_0 = -5.0; // Any negative number will do.
    let delta_f = -delta_0; // Any positive number will do.
    let amplitude = Waveform::interpolated(options.half_duration_ns, &[0., omega, 0.]);
    let detuning = Waveform::interpolated(options.half_duration_ns, &[delta_0, 0f64, delta_f]);
    let channel: Rc<str> = "ising".into();
    Sequence::new(
        device,
        register,
        Pulse::new(channel.clone(), amplitude, detuning),
        &[channel],
    )
}
