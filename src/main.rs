use runner::configure_default;
use runner::list_sensors;
use std::env;

fn cli() {
    println!("Validation Runner is ready to execute commands.");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No input argument provided. Use: 0 to 1");
        return;
    }

    match args[1].as_str() {
        "0" => {
            println!("Running default configuration: configure_default");
            configure_default();
        }
        "1" => {
            println!("Running test 1: list_sensors");
            list_sensors();
        }
        _ => {
            eprintln!("Invalid argument. Expected: 0 to 1");
        }
    }
}

fn main() {
    cli();
}
