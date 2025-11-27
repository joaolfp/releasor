use std::process::{Output, exit};

pub struct Status;

impl Status {
    pub fn print_status(
        release: Output,
        tar: Output,
        shasum: Output,
    ) {
        Self::check(&release, "cargo release");
        Self::check(&tar, "creating tar.gz");
        Self::check_shasum(&shasum);
        
        println!("🎉 All tasks completed successfully!\n");
    }

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