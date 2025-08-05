use std::thread::sleep;
use std::time::Duration;

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
    io::stdin().read_line(&mut input).expect("Failed to read line");
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