use std::process::{Output, exit};

pub struct Status;

impl Status {
    pub fn check(output: &Output, task: &str) {
        if output.status.success() {
            println!("✅ Success {task}");
            return;
        }

        eprintln!(
            "❌ Error {task}\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }

    pub fn check_shasum(output: &Output) {
        if output.status.success() {
            println!(
                "✅ Success get shasum\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
            return;
        }

        eprintln!(
            "❌ Error get shasum\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}
