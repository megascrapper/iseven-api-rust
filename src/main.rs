use std::{fmt::Display, process::exit};

use clap::Parser;
use iseven_api::IsEvenApiBlockingClient;

#[derive(Parser)]
struct Cli {
    /// Number to check
    number: String,

    /// Print JSON response
    #[arg(long)]
    json: bool
}

fn print_error<T: Display>(msg: T) -> ! {
    let argv = std::env::args().collect::<Vec<_>>();
    let app_name = &argv[0];
    eprintln!("error: {}: {}", app_name, msg);
    exit(1)
}

fn main() {
    let cli = Cli::parse();
    let num = cli.number;
    let client = IsEvenApiBlockingClient::new();
    if cli.json {
        match client.get_json(&num) {
            Ok(response) => {
                println!("{}", response)
            }
            Err(e) => {
                print_error(e)
            }
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
            Err(e) => {
                print_error(e)
            }
        }
    }
}
