use releasor::Status;
use std::process::Command;

fn success_output() -> std::process::Output {
    #[cfg(unix)]
    return Command::new("true").output().expect("failed to run true");
    #[cfg(windows)]
    return Command::new("cmd")
        .args(["/C", "exit 0"])
        .output()
        .expect("failed to run exit 0");
}

fn shasum_success_output(stdout: &str) -> std::process::Output {
    #[cfg(unix)]
    return Command::new("echo")
        .arg(stdout)
        .output()
        .expect("failed to run echo");
    #[cfg(windows)]
    return Command::new("cmd")
        .args(["/C", "echo", stdout])
        .output()
        .expect("failed to run echo");
}

#[test]
fn check_success_message_returns_expected_string() {
    let out = success_output();
    let msg = Status::check_success_message(&out, "test task");
    assert_eq!(msg.as_deref(), Some("✅ test task"));
}

#[test]
fn check_success_message_failure_returns_none() {
    #[cfg(unix)]
    let out = std::process::Command::new("false").output().expect("false");
    #[cfg(windows)]
    let out = std::process::Command::new("cmd")
        .args(["/C", "exit", "1"])
        .output()
        .expect("exit 1");
    let msg = Status::check_success_message(&out, "task");
    assert!(msg.is_none());
}

#[test]
fn check_invokes_success_path_without_exiting() {
    let out = success_output();
    Status::check(&out, "test task");
}

#[test]
fn check_quiet_success_does_not_exit() {
    let out = success_output();
    Status::check_quiet(&out, "quiet task");
}

#[test]
fn check_shasum_success_message_returns_expected_string() {
    let out = shasum_success_output("abc123hash  -");
    let msg = Status::check_shasum_success_message(&out);
    assert!(msg.is_some());
    assert_eq!(msg.as_deref(), Some("✅ Get shasum abc123hash  -"));
}

#[test]
fn check_shasum_success_message_trimmed() {
    let out = shasum_success_output("def456  file.tar.gz");
    let msg = Status::check_shasum_success_message(&out);
    assert!(msg.is_some());
    let s = msg.unwrap();
    assert!(s.starts_with("✅ Get shasum "));
    assert!(s.contains("def456"));
}

#[test]
fn check_shasum_invokes_success_path_without_exiting() {
    let out = shasum_success_output("abc123hash  -");
    Status::check_shasum(&out);
}

#[test]
fn check_shasum_quiet_success_does_not_exit() {
    let out = shasum_success_output("def456  file.tar.gz");
    Status::check_shasum_quiet(&out);
}
