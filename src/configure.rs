use crate::commands::run_ccc_command_set_forced;
use crate::misc::*;
use core::str;
use std::io::{self, Write};

pub fn turn_off_imx() {
    run_ccc_command_set_forced("point_enable=0, drive_big_mirror=0");

    // Delay for 500 milliseconds
    timer_sleep_ms(500);

    run_ccc_command_set_forced("imx4_x9_devsts_active=0");

    // Delay for 500 milliseconds
    timer_sleep_ms(500);
}

pub fn turn_on_imx() {
    run_ccc_command_set_forced("imx4_x9_devsts_active=1");

    // Delay for 500 milliseconds
    timer_sleep_ms(500);
}

pub fn turn_on_points() {
    run_ccc_command_set_forced("point_enable=1, drive_big_mirror=1");
}

pub fn turn_off_points() {
    run_ccc_command_set_forced("point_enable=0, drive_big_mirror=0");
}

pub fn set_pixel_format(args: &str) {
    let full_args = format!("pixel_format={}", args);
    run_ccc_command_set_forced(&full_args);
}

pub fn set_mst_freq(args: &str) {
    let full_args = format!("mst_cycle_duration={}", args);
    run_ccc_command_set_forced(&full_args);
}

pub fn turn_on_probing_ffwd() {
    let full_args = format!("probing_ffwd_enabled=1");
    run_ccc_command_set_forced(&full_args);
}

pub fn turn_off_probing_ffwd() {
    let full_args = format!("probing_ffwd_enabled=0");
    run_ccc_command_set_forced(&full_args);
}

pub fn set_src_ip(args: &str) {
    let full_args = format!("src_ip={}", args);
    run_ccc_command_set_forced(&full_args);
}

pub fn configure_default() {
    turn_off_imx();

    // Set default configurations here...
    run_ccc_command_set_forced(
        "nr_slot=960,horiz_fov_left=-1092270,horiz_fov_right=1092270,
        horiz_firing_period=4555,spad_sw_trigger_period=350000,
        spad_trg_enc_dir_invert=0,ambient_data_pkts_enabled=0,
        dynamic_hv_kp=16,dynamic_hv_kc=2100,mst_drive_period=6250,
        mst_half_period=10000000,mst_cycle_duration=200000,mst_phase_offset=0,
        horiz_fov_left_roi=-349525,horiz_fov_right_roi=349525,
        horiz_firing_period_roi=2958,roi_enable=0",
    );

    turn_on_imx();

    println!("Power cycle the sensor");
    println!("Press Enter to continue after the power cycle...");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Sensor is powering up, please wait...");
    timer_countdown(10);
}
