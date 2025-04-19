use std::process::Output;

pub struct Status;

impl Status {

    pub fn get_output_status(
        release: Output,
        tar: Output,
        shasum: Output
    ) {
        if release.status.success() {
            println!("✅ Success cargo release");
        } else {
            eprintln!("❌ Error cargo release");
        }

        if tar.status.success() {
            println!("✅ Success creating tar.gz");
        } else {
            eprintln!("❌ Error creating tar.gz");
        }

        if shasum.status.success() {
            println!("✅ Success get shasum\n");
            println!("{}", String::from_utf8_lossy(&shasum.stdout));
        } else {
            eprintln!("❌ Error get shasum");
        }
    }
}