use ansi_term::Colour::Red;
use human_panic::setup_panic;

use iseven_api::iseven_get;

const USAGE_MSG: &str = "Usage: iseven_api [integer]";

#[tokio::main]
async fn main() {
    setup_panic!();

    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        eprintln!("{} {}", Red.paint("error:"), USAGE_MSG);
    } else {
        let num = &argv[1];
        match iseven_get(num).await {
            Ok(response) => {
                println!("Advertisement: {}", response.ad());
                println!("{} is an {} number", num, if response.iseven() { "even" } else { "odd" })
            }
            Err(e) => eprintln!("{} {}", Red.paint("error:"), e)
        }
    }
}
