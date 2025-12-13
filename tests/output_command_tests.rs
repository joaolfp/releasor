use releasor::OutputCommand;
use std::fs;

#[test]
fn test_release_tar_and_shasum_flow() {
    let out = OutputCommand::cargo_release_output();
    assert!(
        out.status.success(),
        "cargo build --release failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let out = OutputCommand::tar_output("releasor.tar.gz", "releasor");
    assert!(
        out.status.success(),
        "tar failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let out = OutputCommand::get_shasum_output("releasor.tar.gz");
    assert!(
        out.status.success(),
        "shasum failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );

    fs::remove_file("releasor.tar.gz").expect("Failed to delete releasor.tar.gz");
}
