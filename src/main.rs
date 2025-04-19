use releasor::controller::*;

fn main() {
    let project_name = Controller::get_project_name();
    Controller::start_release(project_name);
}