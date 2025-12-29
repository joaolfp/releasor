use std::process::{Output, exit};

pub struct Status;

impl Status {
    pub fn check(output: &Output) {
        if output.status.success() {
            println!("✅ Running cargo release");
            return;
        }

        eprintln!(
            "❌ Error running cargo release {}",
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
            "❌ Error get shasum{}",
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}
