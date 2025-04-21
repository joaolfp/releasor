use std::fs;
use insta::assert_snapshot;
use releasor::OutputCommand;

#[test]
fn test_cargo_release_output_snapshot() {
    let output = OutputCommand::cargo_release_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let snapshot = format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    );

    assert_snapshot!("cargo_build_output", snapshot);
}

#[test]
fn test_tar_output_snapshot() {
    let output = OutputCommand::tar_output(
        "releasor.tar.gz",
        "releasor"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let snapshot = format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    );

    assert_snapshot!("tar_output", snapshot);

    fs::remove_file("releasor.tar.gz")
        .expect("Failed to delete the file");
}