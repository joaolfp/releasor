use std::process::{Output, exit};

pub struct Status;

impl Status {
    /// Returns the success message for `check`, or None if the command failed.
    pub fn check_success_message(output: &Output, task: &str) -> Option<String> {
        if output.status.success() {
            Some(format!("✅ {task}"))
        } else {
            None
        }
    }

    /// Returns the success message for `check_shasum`, or None if the command failed.
    pub fn check_shasum_success_message(output: &Output) -> Option<String> {
        if output.status.success() {
            Some(format!(
                "✅ Get shasum {}",
                String::from_utf8_lossy(&output.stdout).trim_end()
            ))
        } else {
            None
        }
    }

    pub fn check(output: &Output, task: &str) {
        if let Some(msg) = Self::check_success_message(output, task) {
            println!("{msg}");
            return;
        }

        println!();
        eprintln!(
            "❌ Error {task} {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    /// Like `check` but does not print on success (for use with progress bar).
    pub fn check_quiet(output: &Output, task: &str) {
        if output.status.success() {
            return;
        }

        println!();
        eprintln!(
            "❌ Error {task} {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    pub fn check_shasum(output: &Output) {
        if let Some(msg) = Self::check_shasum_success_message(output) {
            println!("{msg}");
            return;
        }

        println!();
        eprintln!(
            "❌ Error get shasum {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    /// Like `check_shasum` but does not print on success (for use with progress bar).
    pub fn check_shasum_quiet(output: &Output) {
        if output.status.success() {
            return;
        }

        println!();
        eprintln!(
            "❌ Error get shasum {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}
