use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use device::Device;
use rand::SeedableRng;

use crate::{
    pulse::{Pulse, Waveform},
    sequence::Sequence,
};

mod device;
mod pulse;
mod pulser;
mod qubo;
mod register;
mod sequence;
mod types;

#[derive(clap::Parser, Debug)]
struct Args {
    /// The file to compile.
    #[arg(long)]
    source: PathBuf,

    /// The device for which to compile.
    #[arg(long)]
    device: PathBuf,

    /// A seed to use for random number generation.
    #[arg(long, default_value_t = 0)]
    seed: u64,

    /// How long we should run the pulse for.
    #[arg(long, default_value_t = 4_000)]
    half_duration_ns: u64,
}

fn main() {
    let args = Args::parse();
    let device = Device::analog();

    let qubo_source = std::fs::File::open(args.source).expect("Failed to open source file");
    let qubo_parsed: qubo::Constraints =
        serde_json::from_reader(qubo_source).expect("Failed to parse source file");

    let rng = rand::rngs::StdRng::seed_from_u64(args.seed);

    // Step: compile the qubo to a register.
    eprintln!("...compiling {} constraints", qubo_parsed.len());
    let (register, quality) = qubo_parsed
        .layout(&device, rng)
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {}",
        register.len(),
        quality
    );

    // Step: integrate QAOA.
    let omega = qubo_parsed.median();
    let delta_0 = -0.5; // Any negative number will do.
    let delta_f = -delta_0; // Any positive number will do.
    let amplitude = Waveform::interpolated(args.half_duration_ns as f64, &[0., omega, 0.]);
    let detuning = Waveform::interpolated(args.half_duration_ns as f64, &[delta_0, 0f64, delta_f]);
    let channel: Arc<str> = "ising".into();
    let sequence = Sequence::new(
        device,
        register,
        Pulse::new(channel.clone(), amplitude, detuning),
        &[channel],
    );
    let json = serde_json::to_string_pretty(&sequence.as_abstract_repr()).unwrap();
    eprintln!("{json}");
}
