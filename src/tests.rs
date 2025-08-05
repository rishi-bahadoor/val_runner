use crate::commands::*;
use crate::configure::*;
use crate::misc::*;

use core::str;

fn test_point_display(args: &str) {
    configure_default();
    set_pixel_format(args);
    turn_on_points();

    println!("Observe the point cloud display.");
    timer_countdown(120);
    turn_off_points();
}

pub fn list_sensors() {
    run_ccc_command("list-sensors");
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
