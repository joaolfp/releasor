use std::process::Output;

pub struct Status;

impl Status {
    pub fn get_output_status(
        release: Output,
        tar: Output,
        shasum: Output
    ) {
        Self::print_result(&release, "cargo release");
        Self::print_result(&tar, "creating tar.gz");

        if shasum.status.success() {
            println!(
                "✅ Success get shasum\n{}", 
                String::from_utf8_lossy(&shasum.stdout)
            );
        } else {
            eprintln!("❌ Error get shasum");
        }
    }

    fn print_result(output: &Output, task: &str) {
        if output.status.success() {
            println!("✅ Success {}", task);
        } else {
            eprintln!("❌ Error {}", task);
        }
    }
}