use releasor::Cli;
use std::process::Command;

#[test]
fn validate_project_name_empty() {
    let r = Cli::validate_project_name("");
    assert!(r.is_err());
    assert_eq!(r.unwrap_err(), "Project name can't be empty");
}

#[test]
fn validate_project_name_whitespace_only() {
    let r = Cli::validate_project_name("   ");
    assert!(r.is_ok(), "trimmed empty is validated by caller");
}

#[test]
fn validate_project_name_contains_slash() {
    let r = Cli::validate_project_name("foo/bar");
    assert!(r.is_err());
    assert_eq!(
        r.unwrap_err(),
        "Project name must not contain path separators"
    );
}

#[test]
fn validate_project_name_contains_backslash() {
    let r = Cli::validate_project_name("foo\\bar");
    assert!(r.is_err());
    assert_eq!(
        r.unwrap_err(),
        "Project name must not contain path separators"
    );
}

#[test]
fn validate_project_name_valid() {
    assert!(Cli::validate_project_name("releasor").is_ok());
    assert!(Cli::validate_project_name("my-project").is_ok());
    assert!(Cli::validate_project_name("a").is_ok());
}

#[test]
fn tar_file_name_format() {
    assert_eq!(Cli::tar_file_name("releasor"), "releasor.tar.gz");
    assert_eq!(Cli::tar_file_name("my-project"), "my-project.tar.gz");
}

#[test]
fn setup_copy_shasum_quiet_parses_first_token() {
    #[cfg(unix)]
    let out = Command::new("echo")
        .arg("abc123def456  -")
        .output()
        .expect("echo");
    #[cfg(windows)]
    let out = Command::new("cmd")
        .args(["/C", "echo", "abc123def456  -"])
        .output()
        .expect("echo");

    let (shasum, _copied) = Cli::setup_copy_shasum_quiet(&out);
    assert_eq!(shasum, "abc123def456");
}

#[test]
fn setup_copy_shasum_quiet_handles_single_line() {
    #[cfg(unix)]
    let out = Command::new("echo").arg("onlyhash").output().expect("echo");
    #[cfg(windows)]
    let out = Command::new("cmd")
        .args(["/C", "echo", "onlyhash"])
        .output()
        .expect("echo");

    let (shasum, _) = Cli::setup_copy_shasum_quiet(&out);
    assert_eq!(shasum, "onlyhash");
}

#[test]
fn setup_copy_shasum_quiet_handles_empty_stdout() {
    #[cfg(unix)]
    let out = Command::new("true").output().expect("true");
    #[cfg(windows)]
    let out = Command::new("cmd")
        .args(["/C", "exit", "0"])
        .output()
        .expect("exit 0");
    let (shasum, _) = Cli::setup_copy_shasum_quiet(&out);
    assert_eq!(shasum, "");
}
