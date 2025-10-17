use std::process::Output;

/// Utility struct for printing and checking the status of executed commands.
pub struct Status;

impl Status {
    /// Prints the status of cargo, tar, and shasum commands and returns `true` if all succeeded.
    pub fn print_status(release: Output, tar: Output, shasum: Output) -> bool {
        let release_ok = Self::print_result(&release, "cargo release");
        let tar_ok = Self::print_result(&tar, "creating tar.gz");
        let shasum_ok = Self::print_shasum(&shasum);

        release_ok && tar_ok && shasum_ok
    }

    /// Prints the result of a single command and returns whether it succeeded.
    fn print_result(output: &Output, task: &str) -> bool {
        if output.status.success() {
            println!("✅ Success {}", task);
            true
        } else {
            eprintln!(
                "❌ Error {}\n{}",
                task,
                String::from_utf8_lossy(&output.stderr)
            );
            false
        }
    }

    /// Prints the result of the shasum command and returns whether it succeeded.
    fn print_shasum(output: &Output) -> bool {
        if output.status.success() {
            println!(
                "✅ Success get shasum\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
            true
        } else {
            eprintln!(
                "❌ Error get shasum\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            false
        }
    }
}