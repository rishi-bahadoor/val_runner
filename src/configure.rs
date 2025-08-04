use crate::commands::run_ccc_command_set_forced;
use std::thread::sleep;
use std::time::Duration;

pub fn turn_off_imx() {
    run_ccc_command_set_forced("point_enable=0, drive_big_mirror=0");

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));

    run_ccc_command_set_forced("imx4_x9_devsts_active=0");

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));
}

pub fn turn_on_imx() {
    run_ccc_command_set_forced("imx4_x9_devsts_active=1");

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));
}

pub fn turn_on_points() {
    turn_on_imx();
    run_ccc_command_set_forced("point_enable=1, drive_big_mirror=1");
}

pub fn turn_off_points() {
    run_ccc_command_set_forced("point_enable=0, drive_big_mirror=0");
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

    println!("Power Cycle the sensor then dp turn on points\n\n")
}
