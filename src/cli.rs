use arboard::Clipboard;
use clap::Parser;

use crate::output_command::OutputCommand;
use crate::progress;
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

        let h = progress::animate_to(25, "Build project");
        Self::run_cargo_release();
        progress::wait_animate(h);

        let h = progress::animate_to(50, "creating tar.gz");
        Self::create_tar_gz(&tar_file, &project_name);
        progress::wait_animate(h);

        let h = progress::animate_to(75, "Get shasum");
        let shasum_output = Self::compute_shasum(&tar_file);
        progress::wait_animate(h);

        let h = progress::animate_to(100, "Copy shasum to clipboard");
        let shasum = Self::setup_copy_shasum_quiet(&shasum_output);
        progress::wait_animate(h);

        progress::show(100, "Done");
        std::thread::sleep(std::time::Duration::from_millis(300));
        progress::finish();

        Self::print_results(&shasum);
    }

    fn parse_and_validate_project_name() -> String {
        let args = Args::parse();
        let project_name = args.file_name.trim().to_string();

        if let Err(msg) = Self::validate_project_name(&project_name) {
            eprintln!("❌ {msg}");
            std::process::exit(1);
        }

        project_name
    }

    /// Validates project name. Returns Ok(()) if valid, Err(message) otherwise.
    pub fn validate_project_name(project_name: &str) -> Result<(), String> {
        if project_name.is_empty() {
            return Err("Project name can't be empty".into());
        }

        if project_name.contains('/') || project_name.contains('\\') {
            return Err("Project name must not contain path separators".into());
        }

        Ok(())
    }

    /// Returns the tar.gz filename for a project name.
    pub fn tar_file_name(project_name: &str) -> String {
        format!("{project_name}.tar.gz")
    }

    fn run_cargo_release() {
        let release = OutputCommand::cargo_release_output();
        Status::check_quiet(&release, "Running cargo release");
    }

    fn create_tar_gz(tar_file: &str, project_name: &str) {
        let tar = OutputCommand::tar_output(tar_file, project_name);
        Status::check_quiet(&tar, "creating tar.gz");
    }

    fn compute_shasum(tar_file: &str) -> std::process::Output {
        let shasum = OutputCommand::get_shasum_output(tar_file);
        Status::check_shasum_quiet(&shasum);
        shasum
    }

    /// Copies shasum to clipboard without printing. Returns the shasum string and clipboard success.
    pub fn setup_copy_shasum_quiet(shasum_output: &std::process::Output) -> (String, bool) {
        let shasum_raw = String::from_utf8_lossy(&shasum_output.stdout);
        let shasum = shasum_raw
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();

        let copied = match Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(shasum.clone()) {
                Ok(()) => true,
                Err(err) => {
                    eprintln!("❌ Failed to copy to clipboard: {err}");
                    false
                }
            },
            Err(err) => {
                eprintln!("❌ Could not access clipboard: {err}");
                false
            }
        };
        (shasum, copied)
    }

    /// Print all results after the progress bar is done.
    fn print_results(shasum: &(String, bool)) {
        let (hash, copied) = shasum;
        println!("✅ Running cargo release");
        println!("✅ creating tar.gz");
        println!("✅ Get shasum {}", hash.trim_end());
        if *copied {
            println!("✅ Shasum copied to clipboard!");
        }
        println!("🎉 All tasks completed successfully!");
    }
}
