use qlafoutea::{
    device::Device,
    pulser::{pulse::Pulse, sequence::Sequence, waveform::Waveform},
};
use std::rc::Rc;

#[test]
fn test_main() {
    let half_duration_ns = 4_000;
    let constraints = qlafoutea::qubo::Constraints::try_new(
        5,
        vec![
            -10.0,
            19.7365809,
            19.7365809,
            5.42015853,
            5.42015853,
            19.7365809,
            -10.0,
            20.67626392,
            0.17675796,
            0.85604541,
            19.7365809,
            20.67626392,
            -10.0,
            0.85604541,
            0.17675796,
            5.42015853,
            0.17675796,
            0.85604541,
            -10.0,
            0.32306662,
            5.42015853,
            0.85604541,
            0.17675796,
            0.32306662,
            -10.0,
        ],
    )
    .unwrap();

    let device = Device::analog();

    eprintln!("...compiling {} constraints", constraints.len());
    let (register, quality) = constraints
        .layout(&device, 0.1, 0)
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {}",
        register.len(),
        quality
    );

    // Step: integrate QAOA.
    let omega = constraints.omega();
    let delta_0 = -5.0; // Any negative number will do.
    let delta_f = -delta_0; // Any positive number will do.
    let amplitude = Waveform::interpolated(half_duration_ns as f64, &[0., omega, 0.]);
    let detuning = Waveform::interpolated(half_duration_ns as f64, &[delta_0, 0f64, delta_f]);
    let channel: Rc<str> = "ising".into();
    let sequence = Sequence::new(
        device,
        register,
        Pulse::new(channel.clone(), amplitude, detuning),
        &[channel],
    );
    let json = serde_json::to_string_pretty(&sequence).unwrap();
    println!("{json}");
}
