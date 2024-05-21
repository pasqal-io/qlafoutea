use std::collections::HashSet;

use qlafoutea::{
    backend::{
        device::Device,
        qaa,
        qubo::{self, Constraints},
    },
    runtime::run::run_source,
    types::Quality,
};

fn qubo_compile() -> String {
    let half_duration_ns = 4_000;
    let constraints = Constraints::try_new(
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

    let (register, quality, _) = constraints
        .layout(
            &device,
            &qubo::Options {
                min_quality: Quality::new(0.1),
                seed: 75,
                max_iters: 1_000,
                overflow_protection_factor: 0.95,
                overflow_protection_threshold: 1_000.,
            },
        )
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {}",
        register.len(),
        quality
    );

    // Step: integrate QAA.
    let sequence = qaa::compile(
        &constraints,
        device,
        register,
        &qaa::Options {
            half_duration_ns: half_duration_ns as f64,
        },
    );

    serde_json::to_string_pretty(&sequence).unwrap()
}

#[test]
fn test_qubo_compile() {
    let json = qubo_compile();
    println!("{json}");
}

#[test]
fn test_qubo_compile_and_run() {
    let json = qubo_compile();
    qlafoutea::runtime::setup().unwrap();
    let samples = run_source(&json).unwrap();

    eprintln!("checking samples {:?}", samples);

    // Compare the best two samples against the known-to-be-best-two
    // samples, as per https://pulser.readthedocs.io/en/stable/tutorials/qubo.html#Quantum-Adiabatic-Algorithm.
    //
    // Note that the order is a bit unstable, so the best two could be
    // swapped.
    let best_two: HashSet<_> = samples
        .iter()
        .take(2)
        .map(|sample| sample.bitstring.as_str())
        .collect();
    let expected_best_two: HashSet<_> = ["00111", "01011"].into_iter().collect();
    assert_eq!(best_two, expected_best_two);
}
