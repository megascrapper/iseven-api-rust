use std::{fmt::Display, process::exit};

use clap::Parser;
use human_panic::setup_panic;
use iseven_api::IsEvenApiBlockingClient;

/// Checks whether a number is even or odd using isEven API (https://isevenapi.xyz/).
#[derive(Parser)]
struct Cli {
    /// Number to check
    numbers: Vec<String>,

    /// Print JSON response
    #[arg(long)]
    json: bool,
}

fn print_error<M: Display, V: Display>(msg: M, input_value: V) -> ! {
    let argv = std::env::args().collect::<Vec<_>>();
    let app_name = &argv[0];
    eprintln!("error: {}: {}: {}", app_name, input_value, msg);
    exit(1)
}

fn main() {
    env_logger::init();
    setup_panic!();
    let cli = Cli::parse();
    let client = IsEvenApiBlockingClient::new();
    for num in cli.numbers {
        if cli.json {
            match client.get_json(&num) {
                Ok(response) => {
                    println!("{}", response)
                }
                Err(e) => print_error(e, num),
            }
        } else {
            match client.get(&num) {
                Ok(response) => {
                    println!("Advertisement: {}", response.ad());
                    println!(
                        "{} is an {} number",
                        &num,
                        if response.iseven() { "even" } else { "odd" }
                    )
                }
                Err(e) => print_error(e, num),
            }
        }
    }
}
