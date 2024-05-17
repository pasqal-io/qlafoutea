use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use itertools::Itertools;
use pyo3::{
    types::{PyAnyMethods, PyDict},
    Bound, Python,
};
use serde::{Deserialize, Serialize};

pub struct Options {
    pub sequence_path: PathBuf,
}

pub fn run(options: Options) -> Result<Vec<Sample>, anyhow::Error> {
    let source = std::fs::read_to_string(options.sequence_path).context("Failed to read input")?;
    run_source(source)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub bitstring: String,
    pub instances: u64,
}

pub fn run_source(source: String) -> Result<Vec<Sample>, anyhow::Error> {
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
    Ok(sorted)
}
