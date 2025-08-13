use std::path::Path;
use std::time::Duration;
use std::{fs, thread::sleep};
use walkdir::WalkDir;

use std::io::{self, Write};

pub fn timer_countdown(seconds: u32) {
    let timer_1 = seconds;
    println!("Timer: {} seconds", timer_1);
    for n in 0..(timer_1 + 1) {
        sleep(Duration::from_millis(1000));
        print!("\r{:3} ", timer_1 - n); // Use padding for consistent width
        io::stdout().flush().unwrap(); // Ensure output is shown immediately
    }
    println!(); // Move to a new line after the loop finishes
}

pub fn timer_sleep_ms(seconds: u32) {
    sleep(Duration::from_millis(seconds as u64));
}

pub fn wait_for_enter() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Continuing...");
}

/// # Example usage
/// ```
/// assert_eq!(runner::hertz_to_cycle_duration(10.0), 200000);
/// assert_eq!(runner::hertz_to_cycle_duration(11.0), 181818);
/// assert_eq!(runner::hertz_to_cycle_duration(20.0), 100000);
/// assert_eq!(runner::hertz_to_cycle_duration(50.0), 40000);
/// ```
pub fn hertz_to_cycle_duration(hz: f64) -> u32 {
    2_000_000 / hz as u32 // Convert Hz to mst_cycle_duration units
}

pub fn get_latest_cepbin_and_toml() {
    const NIGHTLY_PATH: &str = r"\\10.11.0.13\jenkins\ufb\ultra\nightly";

    if !Path::new(NIGHTLY_PATH).exists() {
        eprintln!("Nightly path does not exist: {}", NIGHTLY_PATH);
        eprintln!("Ensure you are connected to the VPN");
        return;
    }

    let latest_release_path = Path::new("./latest_release");

    if !latest_release_path.exists() {
        eprintln!("No file with name latest_release found in the current directory.");
        return;
    }

    let latest_release_content =
        fs::read_to_string(latest_release_path).expect("Failed to read latest_release file");

    let latest_release: u32 = latest_release_content
        .trim()
        .parse()
        .expect("Failed to parse latest_release content as a number");

    println!("Latest release number: {}", latest_release);

    println!(
        "Searching for latest cepbin and toml files in: {}",
        NIGHTLY_PATH
    );

    let dirs: Vec<_> = WalkDir::new(NIGHTLY_PATH)
        .min_depth(1)
        .max_depth(1)
        .sort_by(|a, b| b.file_name().cmp(&a.file_name()))
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_dir())
        .take_while(|entry| {
            let name = entry.file_name().to_string_lossy();
            println!("Checking directory: {}", name);
            let release_number = name.split("#").nth(1);
            if release_number.is_none() {
                return false;
            }
            let release_number = release_number.unwrap().parse().unwrap_or(0);
            release_number >= latest_release
        })
        .collect();

    if dirs.is_empty() {
        eprintln!(
            "No directories found with release number >= {}",
            latest_release
        );
        return;
    }

    let latest_nightly_path = dirs
        .iter()
        .max_by_key(|entry| {
            let name = entry.file_name().to_string_lossy();
            let release_number = name.split("#").nth(1).unwrap_or("0");
            release_number.parse::<u32>().unwrap_or(0)
        })
        .expect("No valid directories found");

    let latest_nightly_version = latest_nightly_path
        .file_name()
        .to_string_lossy()
        .split("#")
        .nth(1)
        .unwrap_or("unknown")
        .parse::<u32>()
        .unwrap_or(0);

    if latest_nightly_version == 0 {
        eprintln!("No valid nightly version found in the latest directory.");
        return;
    }

    if latest_nightly_version == latest_release {
        println!(
            "Latest nightly version {} is the same as the latest release version {}.",
            latest_nightly_version, latest_release
        );
        return;
    }

    let nightly_dir = Path::new("./nightly_latest");
    if nightly_dir.exists() {
        fs::remove_dir_all(nightly_dir).expect("Failed to remove existing nightly_latest directory");
    }
    fs::create_dir(nightly_dir).expect("Failed to create nightly_latest directory");

    for entry in fs::read_dir(latest_nightly_path.path()).expect("Failed to read latest nightly directory") {
        let entry = entry.expect("Failed to read entry");
        let file_name = entry.file_name();
        let file_path = entry.path();

        if file_name.to_string_lossy() == "ultra.cepbin"
            || file_name.to_string_lossy() == "ultra_config.toml"
        {
            let destination = nightly_dir.join(&file_name);
            fs::copy(&file_path, &destination).expect("Failed to copy file");
            println!("Copied {} to {}", file_name.to_string_lossy(), nightly_dir.display());
        }
    }

    fs::write("./nightly_latest/version.txt", latest_nightly_path.file_name().display().to_string())
        .expect("Failed to write version.txt in nightly_latest directory");

    fs::write("./latest_release", latest_nightly_version.to_string())
        .expect("Failed to write latest release number to latest_release file");

    println!("Updated latest_release file with version {}", latest_nightly_version);
}
