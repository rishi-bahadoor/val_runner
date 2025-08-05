use runner::*;
use std::env;

fn cli() {
    println!("Validation Runner is ready to execute commands.");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No input argument provided.");
        return;
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
        _ => {
            eprintln!("Invalid argument.");
        }
    }
}

fn main() {
    cli();
}
