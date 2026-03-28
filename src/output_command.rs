use xx::XXError;
use xx::process;

pub struct OutputCommand;

fn ensure_success(
    ok: bool,
    stderr: &[u8],
    status_when_stderr_empty: impl std::fmt::Display,
) -> xx::XXResult<()> {
    if ok {
        return Ok(());
    }

    let detail = String::from_utf8_lossy(stderr);

    let message = if detail.trim().is_empty() {
        format!("exited with {status_when_stderr_empty}")
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

        ensure_success(output.status.success(), &output.stderr, output.status)?;
        Ok(())
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

        ensure_success(output.status.success(), &output.stderr, output.status)?;
        Ok(())
    }

    pub fn get_shasum(project_tar_gz: &str) -> xx::XXResult<String> {
        let output = process::cmd("shasum", ["-a", "256", project_tar_gz])
            .stdout_capture()
            .stderr_capture()
            .unchecked()
            .run()?;

        ensure_success(output.status.success(), &output.stderr, output.status)?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}
