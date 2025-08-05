use crate::commands::*;
use crate::configure::*;
use crate::misc::*;

use core::str;

// Core functions to run tests
// ==========================================================================
fn test_point_display(args: &str) {
    configure_default();
    set_pixel_format(args);
    turn_on_points();

    println!("Observe the point cloud display.");
    timer_countdown(120);
    turn_off_points();
}

fn test_mst_freq(args: f64) {
    turn_on_points();
    set_mst_freq(args);
    println!("Observe the MST frequency change to {}Hz.", args);
    wait_for_enter();
}

fn test_probing_ffwd(args: &str) {
    set_pixel_format(args);
    turn_on_points();
    turn_on_probing_ffwd();
    if args == "0" {
        println!("Observe the probing forward enabled for STDV format.");
    } else if args == "2" {
        println!("Observe the probing forward enabled for HRAD format.");
    } else if args == "3" {
        println!("Observe the probing forward enabled for HLZ4 format.");
    }
    wait_for_enter();
    turn_off_probing_ffwd();
}

fn test_ip_changes(src_ip: &str, pixel_format: &str, point_on: bool) {
    set_src_ip(src_ip);
    set_pixel_format(pixel_format);
    if point_on {
        turn_on_points();
        println!(
            "Observe packets with source IP {} and pixel format {} and points on.",
            src_ip, pixel_format
        );
    } else {
        turn_off_points();
        println!(
            "Observe packets with source IP {} and pixel format {} and points off.",
            src_ip, pixel_format
        );
    }
    wait_for_enter();
    turn_off_points();
}

// Test functions
// ==========================================================================
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

pub fn test_2_1() {
    test_mst_freq(10.0);
}

pub fn test_2_2() {
    test_mst_freq(20.0);
}

pub fn test_2_3() {
    test_mst_freq(40.0);
}

pub fn test_2_4() {
    test_mst_freq(10.0);
    test_mst_freq(11.0);
    test_mst_freq(12.0);
}

pub fn test_2_5() {
    test_mst_freq(10.0);
    timer_countdown(10);
    test_mst_freq(20.0);
    timer_countdown(10);
}

pub fn test_3_1_1() {
    test_probing_ffwd("0");
}

pub fn test_3_1_2() {
    test_probing_ffwd("2");
}

pub fn test_3_1_3() {
    test_probing_ffwd("3");
}

pub fn test_4_1() {
    run_ccc_update_firmware_with_cepbin("");
}

pub fn test_4_2() {
    run_ccc_boot_server();
}

pub fn test_4_3() {
    run_ccc_update_firmware_with_cepbin("--rescue-mode");
}

pub fn test_4_3_1() {
    run_ccc_update_firmware_nightly("--rescue-mode");
}

pub fn test_6_1_1() {
    test_ip_changes("192.168.32.44", "0", true);
}

pub fn test_6_1_2() {
    test_ip_changes("192.168.32.44", "0", false);
}

pub fn test_6_2_1() {
    test_ip_changes("192.168.32.44", "2", true);
}

pub fn test_6_2_2() {
    test_ip_changes("192.168.32.44", "2", false);
}

pub fn test_6_3_1() {
    test_ip_changes("192.168.32.44", "3", true);
}

pub fn test_6_3_2() {
    test_ip_changes("192.168.32.44", "3", false);
}
