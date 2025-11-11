use std::process::{Output, exit};

/// Utility struct for printing and checking results of executed commands.
pub struct Status;

impl Status {
    /// Prints the status of all commands.
    /// If any command fails, the program terminates immediately.
    pub fn print_status(
        release: Output,
        tar: Output,
        shasum: Output,
    ) {
        Self::check(&release, "cargo release");
        Self::check(&tar, "creating tar.gz");
        Self::check_shasum(&shasum);
        
        println!("🎉 All tasks completed successfully!");
    }

    /// Internal helper: print result or exit on error.
    fn check(output: &Output, task: &str) {
        if output.status.success() {
            println!("✅ Success {}", task);
        } else {
            eprintln!(
                "❌ Error {}\n{}",
                task,
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }
    }

    /// Special-case for shasum, because it prints output on success.
    fn check_shasum(output: &Output) {
        if output.status.success() {
            println!(
                "✅ Success get shasum\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            eprintln!(
                "❌ Error get shasum\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }
    }
}