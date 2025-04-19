use std::io::{self, Write};
use std::process;
use std::process::{Command, Output};

fn main() {
    println!("Get values for homebrew\n");
    let project_name = get_project_name();
    start_release(project_name);
}

fn start_release(project_name: String) {
    let project_tar_gz = format!("{}.tar.gz", project_name);

    let cargo_release_output = cargo_release_output();
    let tar_output = tar_output(&project_tar_gz, &*project_name);
    let get_shasum_output = get_shasum_output(&project_tar_gz);

    validate_output_status(
        cargo_release_output,
        tar_output,
        get_shasum_output,
    );
}

fn get_project_name() -> String {
    print!("What's your project name: ");
    io::stdout().flush().unwrap();

    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read input");

    let project_name = project_name.trim().to_string();

    if project_name.is_empty() {
        eprintln!("❌ Project name can't be empty");
        process::exit(1);
    }

    project_name
}

fn cargo_release_output() -> Output {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .expect("Failed to execute cargo build")
}

fn tar_output(
    project_tar_gz: &str, 
    project_name: &str
) -> Output {
    Command::new("tar")
        .args([
            "-cvzf",
            project_tar_gz,
            "-C",
            "target/release",
            project_name,
        ])
        .output()
        .expect("Failed to create tar.gz")
}

fn get_shasum_output(project_tar_gz: &str) -> Output {
    Command::new("shasum")
        .args(["-a", "256", project_tar_gz])
        .output()
        .expect("Failed to execute shasum")
}

fn validate_output_status(
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