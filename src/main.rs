use ansi_term::Colour::Red;
use std::process::exit;

use iseven_api::IsEven;

const USAGE_MSG: &str = "Usage: iseven_api [integer]";

#[tokio::main]
async fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        eprintln!("{} {}", Red.paint("error:"), USAGE_MSG);
        exit(1);
    } else {
        let num = &argv[1];
        match IsEven::get(num).await {
            Ok(response) => {
                println!("Advertisement: {}", response.ad());
                println!(
                    "{} is an {} number",
                    num,
                    if response.iseven() { "even" } else { "odd" }
                )
            }
            Err(e) => {
                eprintln!("{} {}", Red.paint("error:"), e);
                exit(1);
            }
        }
    }
}
