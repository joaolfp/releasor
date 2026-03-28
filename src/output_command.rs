use xx::process;

pub struct OutputCommand;

impl OutputCommand {
    pub fn cargo_release() -> xx::XXResult<()> {
        process::cmd("cargo", ["build", "--release"])
            .stdout_capture()
            .stderr_capture()
            .run()
            .map(|_| ())
    }

    pub fn tar(project_tar_gz: &str, project_name: &str) -> xx::XXResult<()> {
        process::cmd(
            "tar",
            [
                "-cvzf",
                project_tar_gz,
                "-C",
                "target/release",
                project_name,
            ],
        )
        .stdout_capture()
        .stderr_capture()
        .run()
        .map(|_| ())
    }

    pub fn get_shasum(project_tar_gz: &str) -> xx::XXResult<String> {
        process::cmd("shasum", ["-a", "256", project_tar_gz]).read()
    }
}
