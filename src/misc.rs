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
