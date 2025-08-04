use crate::commands::run_ccc_command_set_forced;
use std::thread::sleep;
use std::time::Duration;

pub fn configure_default() {
    run_ccc_command_set_forced("point_enable=0, drive_big_mirror=0");

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));

    run_ccc_command_set_forced("imx4_x9_devsts_active=0");

    // Set default configurations here...

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));

    run_ccc_command_set_forced("imx4_x9_devsts_active=1");

    // Delay for 500 milliseconds
    sleep(Duration::from_millis(500));

    run_ccc_command_set_forced("point_enable=1, drive_big_mirror=1");
}
