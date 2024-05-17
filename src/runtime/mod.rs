pub mod run;

use anyhow::Context;

/// Install any missing dependencies.
///
/// FIXME: This should be executed only when dependencies are missing.
pub fn setup() -> Result<(), anyhow::Error> {
    let Some(ref executable) = pyo3_build_config::get().executable else {
        return Err(anyhow::anyhow!("Cannot find a Python environment"));
    };
    let mut cmd = std::process::Command::new(executable);
    cmd.args(["-m", "pip", "install", "pulser"]);
    cmd.spawn()
        .context("Failed to launch setup")?
        .wait()
        .context("Error while setting up dependencies")?;
    Ok(())
}
