use std::process::{Command, Output};

/// Utility struct for running common build and packaging commands.
pub struct OutputCommand;

impl OutputCommand {
    /// Runs a shell command with given arguments and returns its output.
    fn run(cmd: &str, args: &[&str], error: &str) -> Output {
        Command::new(cmd)
            .args(args)
            .output()
            .expect(error)
    }

    /// Runs `cargo build --release` and returns the output.
    pub fn cargo_release_output() -> Output {
        Self::run(
            "cargo",
            &["build", "--release"],
            "Failed to execute cargo build",
        )
    }

    /// Creates a `.tar.gz` archive from the release binary.
    pub fn tar_output(project_tar_gz: &str, project_name: &str) -> Output {
        Self::run(
            "tar",
            &["-cvzf", project_tar_gz, "-C", "target/release", project_name],
            "Failed to create tar.gz",
        )
    }

    /// Generates a SHA-256 checksum for the `.tar.gz` file.
    pub fn get_shasum_output(project_tar_gz: &str) -> Output {
        Self::run(
            "shasum",
            &["-a", "256", project_tar_gz],
            "Failed to execute shasum",
        )
    }
}