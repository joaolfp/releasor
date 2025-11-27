use crate::output_command::OutputCommand;
use crate::status::Status;
use arboard::Clipboard;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file_name: String
}

pub struct Controller;

impl Controller {
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
        let tar = OutputCommand::tar_output(&tar_file, project_name);
        let shasum = OutputCommand::get_shasum_output(&tar_file);

        Status::print_status(release, tar, shasum.clone());
        Self::setup_copy_shasum(&shasum);
    }

    fn setup_copy_shasum(shasum_output: &std::process::Output) {
        let shasum = String::from_utf8_lossy(&shasum_output.stdout)
            .trim()
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
