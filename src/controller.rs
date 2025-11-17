use demand::Input;
use std::process;

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
        let input = Input::new("What's your project name?")
            .description("We'll use this to customize the release file name.")
            .placeholder("Enter the project name");
        let name = match input.run() {
            Ok(value) => value.trim().to_string(),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::Interrupted {
                    println!("Input cancelled");
                    process::exit(1);
                } else {
                    panic!("Error: {}", e);
                }
            }
        };

        if name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            process::exit(1);
        }
        if name.contains('/') || name.contains('\\') {
            eprintln!("❌ Project name must not contain path separators");
            process::exit(1);
        }

        name
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
