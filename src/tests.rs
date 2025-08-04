use crate::commands::run_ccc_command;

pub fn list_sensors() {
    run_ccc_command("list-sensors");
}
