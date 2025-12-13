use std::process::{Command, Output};

pub struct OutputCommand;

impl OutputCommand {
    fn run(cmd: &str, args: &[&str], error: &str) -> Output {
        Command::new(cmd).args(args).output().expect(error)
    }

    pub fn cargo_release_output() -> Output {
        Self::run(
            "cargo",
            &["build", "--release"],
            "Failed to execute cargo build",
        )
    }

    pub fn tar_output(project_tar_gz: &str, project_name: &str) -> Output {
        Self::run(
            "tar",
            &[
                "-cvzf",
                project_tar_gz,
                "-C",
                "target/release",
                project_name,
            ],
            "Failed to create tar.gz",
        )
    }

    pub fn get_shasum_output(project_tar_gz: &str) -> Output {
        Self::run(
            "shasum",
            &["-a", "256", project_tar_gz],
            "Failed to execute shasum",
        )
    }
}
