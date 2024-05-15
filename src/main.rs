use std::path::PathBuf;

use clap::Parser;
use qlafoutea::{device::Device, qaa, qubo::format};

#[derive(clap::Parser, Debug)]
struct Args {
    /// The file to compile.
    #[arg(long)]
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
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let device = Device::analog();
    let path_source = args.source.as_path();

    let qubo_source = std::fs::File::open(path_source).expect("Failed to open source file");
    let qubo_parsed = serde_yaml::from_reader::<_, format::Format>(qubo_source)
        .expect("Failed to parse source file");
    let constraints = qubo_parsed.as_constraints();

    // Step: compile the qubo to a register.
    eprintln!("...compiling {} constraints", constraints.len());
    let (register, quality) = constraints
        .layout(&device, args.min_quality, args.seed)
        .expect("Failed to compile qubo");
    eprintln!(
        "...compiled to {} qubits with a quality of {}",
        register.len(),
        quality
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
    let mut path_dest = PathBuf::new();
    path_dest.set_file_name(path_source.file_name().unwrap());
    path_dest = match path_source.extension() {
        None => path_dest.with_extension("pulser.json"),
        Some(ext) => path_dest.with_extension(format!("{}.pulser.json", ext.to_string_lossy())),
    };
    eprintln!("...generating {}", path_dest.display());
    let out_dest = std::fs::File::create(path_dest)?;
    serde_json::to_writer_pretty(out_dest, &sequence)?;
    Ok(())
}
