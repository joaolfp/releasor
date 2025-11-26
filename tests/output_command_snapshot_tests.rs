use std::process::{Output, ExitStatus};
use std::os::unix::process::ExitStatusExt;
use insta::assert_snapshot;
use releasor::output_command::OutputCommandRunner;

struct MockOutputCommand;

impl OutputCommandRunner for MockOutputCommand {
    fn cargo_release_output(&self) -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: b"cargo build --release ouput fake".to_vec(),
            stderr: b"cargo build stderr fake".to_vec(),
        }
    }

    fn tar_output(&self, _project_tar_gz: &str, _project_name: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: b"tar output fake".to_vec(),
            stderr: b"tar stderr fake".to_vec(),
        }
    }

    fn get_shasum_output(&self, _project_tar_gz: &str) -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: b"shasum output fake".to_vec(),
            stderr: b"shasum stderr fake".to_vec(),
        }
    }
}

#[test]
fn test_cargo_release_output_snapshot() {
    let command = MockOutputCommand;
    let output = command.cargo_release_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let snapshot = format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    );

    assert_snapshot!("cargo_build_output", snapshot);
}

#[test]
fn test_tar_output_snapshot() {
    let command = MockOutputCommand;
    let output = command.tar_output("releasor.tar.gz", "releasor");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let snapshot = format!(
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    );

    assert_snapshot!("tar_output", snapshot);
}