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

pub fn test_2_1(){
    set_mst_freq(10.0);
    println!("MST at 10Hz");
    wait_for_enter();
}

pub fn test_2_2(){
    set_mst_freq(20.0);
    println!("MST at 20Hz");
    wait_for_enter();
}

pub fn test_2_3(){
    set_mst_freq(40.0);
    println!("MST at 40Hz");
    wait_for_enter();
}

pub fn test_2_4(){
    set_mst_freq(10.0);
    println!("MST at 10Hz");
    wait_for_enter();
    timer_countdown(10);
    set_mst_freq(11.0);
    println!("MST at 11Hz");
    wait_for_enter();
    timer_countdown(10);
    set_mst_freq(12.0);
    println!("MST at 12Hz");
    wait_for_enter();
}

pub fn test_2_5(){
    set_mst_freq(10.0);
    println!("MST at 10Hz");
    timer_countdown(10);
    set_mst_freq(20.0);
    println!("MST at 20Hz");
    timer_countdown(10);
}

pub fn test_3_1_1(){
    set_pixel_format("0");
    turn_on_probing_ffwd();
    println!("probing_ffwd_enabled for STDV");
    timer_countdown(120);
}

pub fn test_3_1_2(){
    set_pixel_format("2");
    turn_on_probing_ffwd();
    println!("probing_ffwd_enabled for HRAD");
    timer_countdown(120);
}

pub fn test_3_1_3(){
    set_pixel_format("3");
    turn_on_probing_ffwd();
    println!("probing_ffwd_enabled for HLZ4");
    timer_countdown(120);
}

pub fn test_4_1() {
    run_ccc_update_firmware_with_cepbin("");
    println!("ccc update-firmware");
}

pub fn test_4_2() {
    run_ccc_boot_server();
    println!("ccc boot-server");
}

pub fn test_4_3() {
    run_ccc_update_firmware_with_cepbin("--rescue-mode");
    println!("ccc update-firmware --rescue-mode");
}

pub fn test_4_3_1() {
    run_ccc_update_firmware_nightly("--rescue-mode");
    println!("ccc update-firmware --rescue-mode --nightly");
}

pub fn test_6_1_1() {
    set_src_ip("192.168.32.44");
    set_pixel_format("0");
    turn_on_points();
    println!("src_ip=192.168.32.44 for STDV format point cloud on");
}

pub fn test_6_1_2() {
    set_src_ip("192.168.32.44");
    set_pixel_format("0");
    turn_off_points();
    println!("src_ip=192.168.32.44 for STDV format point cloud off");
}

pub fn test_6_2_1() {
    set_src_ip("192.168.32.44");
    set_pixel_format("2");
    turn_on_points();
    println!("src_ip=192.168.32.44 for HRAD format point cloud on");
}

pub fn test_6_2_2() {
    set_src_ip("192.168.32.44");
    set_pixel_format("2");
    turn_off_points();
    println!("src_ip=192.168.32.44 for HRAD format point cloud off");
}

pub fn test_6_3_1() {
    set_src_ip("192.168.32.44");
    set_pixel_format("3");
    turn_on_points();
    println!("src_ip=192.168.32.44 for HLZ4 format point cloud on");
}

pub fn test_6_3_2() {
    set_src_ip("192.168.32.44");
    set_pixel_format("3");
    turn_off_points();
    println!("src_ip=192.168.32.44 for HLZ4 format point cloud off");
}
