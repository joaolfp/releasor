use arboard::Clipboard;
use clap::Parser;

use crate::output_command::OutputCommand;
use crate::status::Status;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

pub struct Cli;

impl Cli {
    pub fn start_release() {
        Self::generate_tar_gz();
    }

    fn generate_tar_gz() {
        let project_name = Self::parse_and_validate_project_name();
        let tar_file = Self::tar_file_name(&project_name);

        Self::run_cargo_release();
        Self::create_tar_gz(&tar_file, &project_name);

        let shasum_output = Self::compute_shasum(&tar_file);
        Self::setup_copy_shasum(&shasum_output);

        Self::print_success();
    }

    fn parse_and_validate_project_name() -> String {
        let args = Args::parse();
        let project_name = args.file_name.trim().to_string();

        Self::validate_project_name(&project_name);

        project_name
    }

    fn validate_project_name(project_name: &str) {
        if project_name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            std::process::exit(1);
        }

        if project_name.contains('/') || project_name.contains('\\') {
            eprintln!("❌ Project name must not contain path separators");
            std::process::exit(1);
        }
    }

    fn tar_file_name(project_name: &str) -> String {
        format!("{project_name}.tar.gz")
    }

    fn run_cargo_release() {
        let release = OutputCommand::cargo_release_output();
        Status::check(&release, "Running cargo release");
    }

    fn create_tar_gz(tar_file: &str, project_name: &str) {
        let tar = OutputCommand::tar_output(tar_file, project_name);
        Status::check(&tar, "creating tar.gz");
    }

    fn compute_shasum(tar_file: &str) -> std::process::Output {
        let shasum = OutputCommand::get_shasum_output(tar_file);
        Status::check_shasum(&shasum);
        shasum
    }

    fn print_success() {
        println!("🎉 All tasks completed successfully!");
    }

    fn setup_copy_shasum(shasum_output: &std::process::Output) {
        let shasum_raw = String::from_utf8_lossy(&shasum_output.stdout);
        let shasum = shasum_raw
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();

        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(err) = clipboard.set_text(shasum.clone()) {
                    eprintln!("❌ Failed to copy to clipboard: {err}");
                } else {
                    println!("✅ Shasum copied to clipboard!");
                }
            }
            Err(err) => {
                eprintln!("❌ Could not access clipboard: {err}");
            }
        }
    }
}
