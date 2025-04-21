use std::{io, process};
use std::io::Write;

use crate::output_command::OutputCommand;
use crate::status::*;

pub struct Controller;

impl Controller {

    pub fn start_release() {
        println!("Get values for homebrew\n");
        let project_name = Controller::get_project_name();
        Self::generate_tar_gz(&project_name);
    }

    fn get_project_name() -> String {
        print!("What's your project name: ");
        io::stdout().flush().unwrap();

        let mut project_name = String::new();
        io::stdin()
            .read_line(&mut project_name)
            .expect("Failed to read input");

        let project_name = project_name.trim().to_string();

        if project_name.is_empty() {
            eprintln!("❌ Project name can't be empty");
            process::exit(1);
        }

        project_name
    }
    
    fn generate_tar_gz(project_name: &str) {
        let project_tar_gz = format!("{}.tar.gz", project_name);

        let cargo_release_output = OutputCommand::cargo_release_output();
        let tar_output = OutputCommand::tar_output(&project_tar_gz, &*project_name);
        let get_shasum_output = OutputCommand::get_shasum_output(&project_tar_gz);

        Status::get_output_status(
            cargo_release_output,
            tar_output,
            get_shasum_output,
        );
    }
}