use std::{io, process};
use std::io::Write;

use crate::output_command::OutputCommand;
use crate::status::Status;

/// Orchestrates the release workflow: prompt, build, archive, checksum, and report.
pub struct Controller;

impl Controller {
    /// Starts the release flow and exits with code 1 if any step fails.
    pub fn start_release() {
        println!("Preparing Homebrew artifacts\n");
        let project_name = Self::get_project_name();
        Self::generate_tar_gz(project_name.as_str());
    }

    /// Prompts the user for the project name and returns a validated non-empty string.
    fn get_project_name() -> String {
        print!("What's your project name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let name = input.trim();

        if name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            process::exit(1);
        }
        if name.contains('/') || name.contains('\\') {
            eprintln!("❌ Project name must not contain path separators");
            process::exit(1);
        }

        name.to_string()
    }

    /// Builds the release binary, creates `<name>.tar.gz`, computes SHA-256, and prints status.
    fn generate_tar_gz(project_name: &str) {
        let tar_file = format!("{}.tar.gz", project_name);

        let release = OutputCommand::cargo_release_output();
        let tar = OutputCommand::tar_output(&tar_file, project_name);
        let shasum = OutputCommand::get_shasum_output(&tar_file);

        Status::print_status(release, tar, shasum);
    }
}