use crate::commands::*;
use crate::configure::*;

use core::str;
use std::thread::sleep;
use std::time::Duration;

use std::io::{self, Write};

pub fn list_sensors() {
    run_ccc_command("list-sensors");
}

pub fn test_point_display(args: &str) {
    configure_default();
    set_pixel_format(args);
    turn_on_points();

    println!("Observe the point cloud display.");
    let timer_2 = 120;
    println!("Timer: {} seconds", timer_2);
    for n in 0..timer_2 {
        sleep(Duration::from_millis(1000));
        print!("\r{:3} ", timer_2 - n); // Use padding for consistent width
        io::stdout().flush().unwrap(); // Ensure output is shown immediately
    }
    println!(); // Move to a new line after the loop finishes
}

pub fn test_1_1() {
    test_point_display("0");
}

pub fn test_1_2() {
    test_point_display("2");
}

pub fn test_1_3() {
    test_point_display("3");
}
