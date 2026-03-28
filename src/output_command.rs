use std::process::Output;

use xx::process;
use xx::XXError;

pub struct OutputCommand;

fn fail_if_unsuccessful(output: Output) -> xx::XXResult<()> {
    if output.status.success() {
        return Ok(());
    }

    let detail = String::from_utf8_lossy(&output.stderr);

    let message = if detail.trim().is_empty() {
        format!("exited with {status}", status = output.status)
    } else {
        detail.into_owned()
    };

    Err(XXError::Error(message))
}

impl OutputCommand {
    pub fn cargo_release() -> xx::XXResult<()> {
        let output = process::cmd("cargo", ["build", "--release"])
            .stdout_capture()
            .stderr_capture()
            .unchecked()
            .run()?;

        fail_if_unsuccessful(output)
    }

    pub fn tar(project_tar_gz: &str, project_name: &str) -> xx::XXResult<()> {
        let output = process::cmd(
            "tar",
            [
                "-cvzf",
                project_tar_gz,
                "-C",
                "target/release",
                project_name,
            ],
        )
        .stdout_capture()
        .stderr_capture()
        .unchecked()
        .run()?;

        fail_if_unsuccessful(output)
    }

    pub fn get_shasum(project_tar_gz: &str) -> xx::XXResult<String> {
        process::cmd("shasum", ["-a", "256", project_tar_gz]).read()
    }
}
