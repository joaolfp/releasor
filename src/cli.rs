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
        let arg = Args::parse();
        let project_name = arg.file_name.trim();

        if project_name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            std::process::exit(1);
        }

        if project_name.contains('/') || project_name.contains('\\') {
            eprintln!("❌ Project name must not contain path separators");
            std::process::exit(1);
        }

        let tar_file = format!("{}.tar.gz", project_name);

        let release = OutputCommand::cargo_release_output();
        Status::check(&release, "cargo release");

        let tar = OutputCommand::tar_output(&tar_file, project_name);
        Status::check(&tar, "creating tar.gz");

        let shasum = OutputCommand::get_shasum_output(&tar_file);
        Status::check_shasum(&shasum);

        println!("🎉 All tasks completed successfully!\n");
        Self::setup_copy_shasum(&shasum);
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
