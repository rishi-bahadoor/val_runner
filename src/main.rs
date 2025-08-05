use runner::*;
use std::env;
use std::sync::atomic::Ordering::Relaxed;

fn cli() {
    println!("Validation Runner is ready to execute commands.");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("No input argument provided.");
        return;
    }

    if args.len() == 3 && (args[2] == "verbose" || args[2] == "-v" || args[2] == "--verbose") {
        println!("Running in verbose mode.");
        VERBOSE.store(true, Relaxed);
    }

    match args[1].as_str() {
        "0" => {
            println!("Default configuration");
            configure_default();
        }
        "1" => {
            println!("list sensors");
            list_sensors();
        }
        "2" => {
            println!("Turn on points");
            turn_on_points();
        }
        "3" => {
            println!("Turn off points");
            turn_off_points();
        }
        "4" => {
            println!("Turn on imx");
            turn_on_imx();
        }
        "5" => {
            println!("Turn off imx");
            turn_off_imx();
        }
        "6" => {
            println!("Get all");
            run_ccc_command_get_all();
        }
        "7" => {
            turn_on_probing_ffwd();
        }
        "8" => {
            turn_off_probing_ffwd();
        }
        "1.1" => {
            println!("1.1");
            test_1_1();
        }
        "1.2" => {
            println!("1.2");
            test_1_2();
        }
        "1.3" => {
            println!("1.3");
            test_1_3();
        }
        "2.1" => {
            println!("2.1");
            test_2_1();
        }
        "2.2" => {
            println!("2.2");
            test_2_2();
        }
        "2.3" => {
            println!("2.3");
            test_2_3();
        }
        "2.4" => {
            println!("2.4");
            test_2_4();
        }
        "2.5" => {
            println!("2.5");
            test_2_5();
        }
        "3.1.1" => {
            println!("3.1.1");
            test_3_1_1();
        }
        "3.1.2" => {
            println!("3.1.2");
            test_3_1_2();
        }
        "3.1.3" => {
            println!("3.1.3");
            test_3_1_3();
        }
        "4.1" => {
            println!("4.1");
            test_4_1();
        }
        "4.2" => {
            println!("4.2");
            test_4_2();
        }
        "4.3" => {
            println!("4.3");
            test_4_3();
        }
        "4.3.1" => {
            println!("4.3.1");
            test_4_3_1();
        }
        "6.1.1" => {
            println!("6.1.1");
            test_6_1_1();
        }
        "6.1.2" => {
            println!("6.1.2");
            test_6_1_2();
        }
        "6.2.1" => {
            println!("6.2.1");
            test_6_2_1();
        }
        "6.2.2" => {
            println!("6.2.2");
            test_6_2_2();
        }
        "6.3.1" => {
            println!("6.3.1");
            test_6_3_1();
        }
        "6.3.2" => {
            println!("6.3.2");
            test_6_3_2();
        }
        _ => {
            eprintln!("Invalid argument.");
        }
    }
}

fn main() {
    cli();
}
