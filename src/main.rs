use std::path::PathBuf;

use clap::Parser;
use qlafoutea::{
    backend::{
        device::Device,
        qaa,
        qubo::{self, format},
    },
    path::PathExt,
    runtime,
    types::Quality,
};

/// Build from QUBO intermediate language.
///
/// Once we have front-end capabilities, this command will serve mainly for debugging.
#[derive(clap::Parser, Debug)]
struct Backend {
    /// The file to compile.
    source: PathBuf,

    /// A seed to use for random number generation.
    #[arg(long, default_value_t = 0)]
    seed: u64,

    /// The minimal quality to accept for compilation.
    #[arg(long, default_value_t = 0.2)]
    min_quality: f64,

    /// How long we should run the pulse for.
    #[arg(long, default_value_t = 4_000)]
    half_duration_ns: u64,

    /// An upper bound to the number of iterations in each attempt we make to find an optimal
    /// register.
    #[arg(long, default_value_t = 4_000)]
    max_iters: u64,
}

#[derive(clap::Parser, Debug)]
struct Run {
    /// The file to run.
    ///
    /// It must have been compiled already.
    source: PathBuf,
}

#[derive(Debug, Parser)]
enum Command {
    /// Build from QUBO intermediate language.
    Backend(Backend),

    /// Launch a previously built program.
    Run(Run),
}

fn backend(args: Backend) -> Result<(), anyhow::Error> {
    let device = Device::analog();
    let path_source = args.source.as_path();

    let qubo_source = std::fs::File::open(path_source).expect("Failed to open source file");
    let qubo_parsed = serde_yaml::from_reader::<_, format::Format>(qubo_source)
        .expect("Failed to parse source file");
    let constraints = qubo_parsed.as_constraints();

    // Step: compile the qubo to a register.
    eprintln!("...compiling {} constraints", constraints.len());
    let (register, quality, seed) = constraints
        .layout(
            &device,
            &qubo::Options {
                seed: args.seed,
                min_quality: Quality::new(args.min_quality),
                max_iters: args.max_iters,
            },
        )
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {} (using seed {})",
        register.len(),
        quality,
        seed
    );

    // Step: integrate QAA.
    let sequence = qaa::compile(
        constraints,
        device,
        register,
        &qaa::Options {
            half_duration_ns: args.half_duration_ns as f64,
        },
    );

    // Write pulser output.
    // In the future, we'll probably write more data in the file.
    let path_dest = path_source.here_with_ext("pulser.json");
    eprintln!("...generating {}", path_dest.display());
    let out_dest = std::fs::File::create(path_dest)?;
    serde_json::to_writer_pretty(out_dest, &sequence)?;
    Ok(())
}

fn run(args: Run) -> Result<(), anyhow::Error> {
    let path_dest = args.source.here_with_ext("samples.csv");

    eprintln!("...setting up emulator");
    runtime::setup()?;
    eprintln!("...starting emulation");
    let result = runtime::run::run(runtime::run::Options {
        sequence_path: args.source,
    })?;

    eprintln!("...storing samples in {}", path_dest.display());
    let mut writer = csv::Writer::from_path(path_dest)?;
    for record in result {
        writer.serialize(record)?;
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Command::parse();
    match args {
        Command::Backend(args) => backend(args),
        Command::Run(args) => run(args),
    }
}
