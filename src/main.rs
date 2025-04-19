use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Get values for homebrew\n");

    // Prompt user for project name
    print!("What's your project name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim(); // removes newline

    let program_tar_gz = format!("{}.tar.gz", name);

    // Run `cargo build --release`
    let build_output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .expect("Failed to execute cargo build");

    println!(
        "cargo build stdout:\n{}",
        String::from_utf8_lossy(&build_output.stdout)
    );
    eprintln!(
        "cargo build stderr:\n{}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Run `mise release`
    let release = Command::new("mise")
        .arg("release")
        .output()
        .expect("Failed to execute mise release");

    println!("mise release status: {}", release.status);

    // Create .tar.gz from built binary
    let tar_output = Command::new("tar")
        .args([
            "-cvzf",
            &program_tar_gz,
            "-C",
            "target/release",
            name
        ])
        .output()
        .expect("Failed to create tar.gz");

    if tar_output.status.success() {
        println!("✅ Success creating {}", program_tar_gz);
    } else {
        eprintln!("❌ Error creating tar.gz");
    }

    // TODO: I need to implement the status for release
    // TODO: Change variable names
    // TODO: Organize into functions
    // TODO: I need to validate in case of empty field
}
