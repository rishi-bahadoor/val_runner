use std::process::Command;

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
    }
}

pub fn run_ccc_command_with_toml(args: &str) {
    let powershell_script = format!(
        "cd '{}'; ./ccc.exe {} -d {}",
        PATH_TO_CCC_EXE, args, PATH_TO_TOML
    );

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
    }
}

pub fn run_ccc_command_with_toml_and_forced(args: &str) {
    let powershell_script = format!(
        "cd '{}'; ./ccc.exe {} -d {} --force",
        PATH_TO_CCC_EXE, args, PATH_TO_TOML
    );

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
    }
}
