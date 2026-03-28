use releasor::OutputCommand;
use std::fs;

#[test]
fn test_release_tar_and_shasum_flow() {
    OutputCommand::cargo_release().expect("cargo build --release failed");

    OutputCommand::tar("releasor.tar.gz", "releasor").expect("tar failed");

    let shasum = OutputCommand::get_shasum("releasor.tar.gz").expect("shasum failed");
    assert!(!shasum.is_empty(), "shasum output should not be empty");

    fs::remove_file("releasor.tar.gz").expect("Failed to delete releasor.tar.gz");
}
