use std::process::Command;

// test

const PATH_TO_CCC_EXE: &str = r"C:\Users\rishi\Downloads\validation_runner\runner";
const PATH_TO_TOML: &str = r"C:\Users\rishi\Downloads\validation_runner\runner\ultra_config.toml";

pub fn run_ccc_command(args: &str) {
    let powershell_script = format!("cd '{}'; ./ccc.exe {}", PATH_TO_CCC_EXE, args);

    let output = Command::new("powershell")
        .args(["-Command", &powershell_script])
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
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
    let full_args = format!("set {} -d {} --force", args, PATH_TO_TOML);
    run_ccc_command(&full_args);
}

pub fn run_ccc_command_get(args: &str) {
    let full_args = format!("get {} -d {}", args, PATH_TO_TOML);
    run_ccc_command(&full_args);
}
