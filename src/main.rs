use std::path::PathBuf;

use clap::Parser;
use qlafoutea::{
    backend::{device::Device, format::Code, qaa, qubo},
    path::PathExt,
    runtime,
    types::Quality,
};

#[derive(clap::Parser, Debug)]
struct Build {
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

    /// While laying out qubo, we make it costly to place atoms too close to the physical limits
    /// of the device. This value determines how much we worry when atoms are laid out in the
    /// unacceptable zone.
    #[arg(long, default_value_t = 1_000.)]
    overflow_protection_factor: f64,

    /// While laying out qubo, we make it costly to place atoms too close to the physical limits
    /// of the device. This value determines how close is acceptable, with `0` meaning "start
    /// worrying immediately` and `1` meaning "start worrying only if the atoms are beyond the
    /// physical limits of the device".
    #[arg(long, default_value_t = 0.95)]
    overflow_protection_threshold: f64,
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
    /// Build from high-level code.
    Build(Build),

    /// Launch a previously built program.
    Run(Run),
}

fn build(args: Build) -> Result<(), anyhow::Error> {
    let device = Device::analog();
    let path_source = args.source.as_path();

    // Step: parse source.
    let source = std::fs::File::open(path_source).expect("Failed to open source file");
    let problem = serde_yaml::from_reader::<_, qlafoutea::frontend::Input>(source)
        .expect("Failed to parse source file");

    // Step: compile to qubo.
    let constraints = problem.to_constraints().expect("Failed to compile to QUBO");

    // Step: compile the qubo to a register.
    eprintln!("...compiling {} constraints", constraints.num_constraints());
    let (register, quality, seed) = constraints
        .layout(
            &device,
            &qubo::Options {
                seed: args.seed,
                min_quality: Quality::new(args.min_quality),
                max_iters: args.max_iters,
                overflow_protection_factor: args.overflow_protection_factor,
                overflow_protection_threshold: args.overflow_protection_threshold,
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
        &constraints,
        device,
        register,
        &qaa::Options {
            half_duration_ns: args.half_duration_ns as f64,
        },
    );

    // Step: write "bytecode".
    let code = Code::try_new(problem, sequence).expect("Couldn't generate code");

    // Write pulser output.
    // In the future, we'll probably write more data in the file.
    let path_dest = path_source.here_with_ext("qlaf");
    eprintln!("...generating {}", path_dest.display());
    let out_dest = std::fs::File::create(path_dest)?;
    serde_json::to_writer_pretty(out_dest, &code)?;
    Ok(())
}

fn run(args: Run) -> Result<(), anyhow::Error> {
    eprintln!("...loading code");
    let input = std::fs::File::open(args.source).expect("Failed to open code");
    let code: Code = serde_yaml::from_reader(input).expect("Failed to parse code");

    eprintln!("...setting up emulator");
    runtime::setup()?;

    eprintln!("...starting emulation");
    runtime::run::run(code)?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Command::parse();
    match args {
        Command::Build(args) => build(args),
        Command::Run(args) => run(args),
    }
}
