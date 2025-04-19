use std::process::{Command, Output};

pub struct OutputCommand;

impl OutputCommand {

    pub fn cargo_release_output() -> Output {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .output()
            .expect("Failed to execute cargo build")
    }

    pub fn tar_output(
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

    pub fn get_shasum_output(project_tar_gz: &str) -> Output {
        Command::new("shasum")
            .args(["-a", "256", project_tar_gz])
            .output()
            .expect("Failed to execute shasum")
    }
}