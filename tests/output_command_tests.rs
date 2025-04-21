use std::fs;
use releasor::OutputCommand;

#[test]
fn test_cargo_release_output() {
    let output = OutputCommand::cargo_release_output();
    let status = output.status;
    assert_eq!(status.success(), true);
}

#[test]
fn test_tar_output() {
    let output = OutputCommand::tar_output(
        "releasor.tar.gz", 
        "releasor"
    );

    let status = output.status;
    assert_eq!(status.success(), true);
}

#[test]
fn test_get_shasum_output() {
    let output = OutputCommand::get_shasum_output(
        "releasor.tar.gz"
    );

    let status = output.status;
    assert_eq!(status.success(), true);

    fs::remove_file("releasor.tar.gz")
        .expect("Failed to delete the file");
}