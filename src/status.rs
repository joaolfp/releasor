use std::process::{Output, exit};

pub struct Status;

impl Status {
    pub fn check(output: &Output, task: &str) {
        if output.status.success() {
            println!("✅ {task}");
            return;
        }

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

        eprintln!(
            "❌ Error {task} {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    pub fn check_shasum(output: &Output) {
        if output.status.success() {
            println!(
                "✅ Get shasum {}",
                String::from_utf8_lossy(&output.stdout).trim_end()
            );
            return;
        }

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

        eprintln!(
            "❌ Error get shasum {}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}
