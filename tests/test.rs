use qlafoutea::{device::Device, qaa, qubo, types::Quality};

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
        .layout(
            &device,
            &qubo::Options {
                min_quality: Quality::new(0.1),
                seed: 75,
                max_iters: 1_000,
            },
        )
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {}",
        register.len(),
        quality
    );

    // Step: integrate QAOA.
    let sequence = qaa::compile(
        &constraints,
        device,
        register,
        &qaa::Options {
            half_duration_ns: half_duration_ns as f64,
        },
    );

    let json = serde_json::to_string_pretty(&sequence).unwrap();
    println!("{json}");
}
