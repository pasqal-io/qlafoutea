use std::collections::HashMap;

use anyhow::Context;
use itertools::Itertools;
use pyo3::{
    types::{PyAnyMethods, PyDict},
    Bound, Python,
};
use serde::{Deserialize, Serialize};

use crate::{
    backend::{format::Code, pulser::sequence::Sequence},
    studio,
};

pub enum Runner {
    PyPulser,
    PulserStudio,
}

pub struct Options {
    /// How many results to display.
    ///
    /// If this value is in [0., 1.], discard any result
    /// if the number of samples in which it appears is <
    /// best result * result_sample_threshold.
    pub result_sample_threshold: f64,

    pub runner: Runner,
}

pub fn run(code: Code, options: Options) -> Result<(), anyhow::Error> {
    let mut sorted_samples = match options.runner {
        Runner::PyPulser => run_python(&code.sequence)?,
        Runner::PulserStudio => run_studio(&code.sequence)?,
    };

    // Only keep the best entries.
    let maybe_cut_at = if let Some(best) = sorted_samples.first() {
        if let Some((first, _)) = sorted_samples.iter().find_position(|sample| {
            (sample.instances as f64 / best.instances as f64) < options.result_sample_threshold
        }) {
            Some(first)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(cut_at) = maybe_cut_at {
        sorted_samples.resize_with(cut_at, || panic!());
    }

    code.problem.handle_results(&sorted_samples)?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub bitstring: String,
    pub instances: u64,
}

pub fn run_python(source: &str) -> Result<Vec<Sample>, anyhow::Error> {
    let Some(ref executable) = pyo3_build_config::get().executable else {
        return Err(anyhow::anyhow!("Cannot find a Python environment"));
    };
    let mut cmd = std::process::Command::new(executable);
    cmd.args(["-m", "pip", "install", "pulser"]);
    cmd.spawn()
        .context("Failed to launch setup")?
        .wait()
        .context("Error while setting up dependencies")?;

    pyo3::prepare_freethreaded_python();
    let samples = Python::with_gil(|py| -> Result<HashMap<String, u64>, pyo3::PyErr> {
        let sequence_builder = py.import_bound("pulser")?.getattr("Sequence")?;
        let qutip_emulator = py
            .import_bound("pulser_simulation")?
            .getattr("QutipEmulator")?;
        let sequence = sequence_builder.call_method1("from_abstract_repr", (source,))?;
        let simulator = qutip_emulator.call_method1("from_sequence", (sequence,))?;
        let result = simulator.call_method0("run")?;
        let np_samples: Bound<PyDict> = result.call_method0("sample_final_state")?.extract()?;

        let mut samples = HashMap::new();
        for (k, v) in np_samples {
            let k: String = k.extract()?;
            let v: u64 = v.extract()?;
            samples.insert(k, v);
        }
        Ok(samples)
    })
    .context("Failed to run simulator")?;
    let sorted = samples
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .map(|(bitstring, instances)| Sample {
            bitstring,
            instances,
        })
        .collect_vec();

    eprintln!("simulation complete");
    Ok(sorted)
}

pub fn run_studio(source: &str) -> Result<Vec<Sample>, anyhow::Error> {
    let sequence: Sequence = serde_json::from_str(source).context("Invalid sequence")?;
    let studio = studio::Runner::new()?;
    let mut simulator = studio.simulator()?;
    simulator.simulate_sequence(sequence)?;
    unimplemented!()
}
