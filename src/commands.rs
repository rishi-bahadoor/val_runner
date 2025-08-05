use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);

const PATH_TO_CCC_EXE: &str = r"..\val_runner";
const PATH_TO_TOML: &str = r"..\val_runner\ultra_config.toml";
const PATH_TO_CEPBIN: &str = r"..\val_runner\ultra.cepbin";

pub fn run_ccc_command(args: &str) {
    let powershell_script = format!("cd '{}'; ./ccc.exe {}", PATH_TO_CCC_EXE, args);

    if VERBOSE.load(Relaxed) {
        println!("Running command: {}", powershell_script);
    }

    let mut child = Command::new("powershell")
        .args(["-Command", &powershell_script])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start PowerShell process");

    let status = child.wait().expect("Failed to wait on PowerShell process");

    if !status.success() {
        eprintln!("PowerShell command failed with status: {}", status);
    }
}

pub fn run_ccc_command_with_toml(args: &str) {
    let full_args = format!("{} -d {}", args, PATH_TO_TOML);
    run_ccc_command(&full_args);
}

pub fn run_ccc_command_with_toml_and_forced(args: &str) {
    let full_args = format!("{} -d {} --force", args, PATH_TO_TOML);
    run_ccc_command(&full_args);
}

pub fn run_ccc_command_set_forced(args: &str) {
    let full_args = format!("set {}", args);
    run_ccc_command_with_toml_and_forced(&full_args);
}

pub fn run_ccc_command_get(args: &str) {
    let full_args = format!("get {}", args);
    run_ccc_command_with_toml(&full_args);
}

pub fn run_ccc_command_get_all() {
    let full_args = format!("get-all");
    run_ccc_command_with_toml(&full_args);
}

pub fn run_ccc_update_firmware_with_cepbin(args: &str) {
    let full_args = format!("update-firmware {} {}", PATH_TO_CEPBIN, args);
    run_ccc_command(&full_args);
}

pub fn run_ccc_update_firmware_nightly(args: &str) {
    let full_args = format!("update-firmware {} --nightly", args);
    run_ccc_command(&full_args);
}

pub fn run_ccc_boot_server() {
    let full_args = format!("boot-server {}", PATH_TO_CEPBIN);
    run_ccc_command(&full_args);
}
