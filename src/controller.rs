use std::{io, process};
use std::io::Write;

use crate::output_command::OutputCommand;
use crate::status::Status;

pub struct Controller;

impl Controller {
    pub fn start_release() {
        println!("Get values for homebrew\n");
        let project_name = Self::get_project_name();
        Self::generate_tar_gz(project_name.as_str());
    }

    fn get_project_name() -> String {
        print!("What's your project name: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let name = input.trim();

        if name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            process::exit(1);
        }

        name.to_string()
    }
    
    fn generate_tar_gz(project_name: &str) {
        let tar_file = format!("{}.tar.gz", project_name);

        let release = OutputCommand::cargo_release_output();
        let tar = OutputCommand::tar_output(&tar_file, project_name);
        let shasum = OutputCommand::get_shasum_output(&tar_file);

        Status::get_output_status(release, tar, shasum);
    }
}