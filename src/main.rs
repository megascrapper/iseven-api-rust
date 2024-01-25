use std::process::exit;

use iseven_api::IsEvenApiBlockingClient;

const USAGE_MSG: &str = "Usage: iseven_api [integer]";

fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    let app_name = &argv[0];
    if argv.len() != 2 {
        eprintln!("error: {}: {}", app_name, USAGE_MSG);
        exit(1);
    } else {
        let num = &argv[1];
        let client = IsEvenApiBlockingClient::new();
        match client.get(num) {
            Ok(response) => {
                println!("Advertisement: {}", response.ad());
                println!(
                    "{} is an {} number",
                    num,
                    if response.iseven() { "even" } else { "odd" }
                )
            }
            Err(e) => {
                eprintln!("error: {}: {}", app_name, e);
                exit(1);
            }
        }
    }
}
