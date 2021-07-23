use iseven_api::iseven_get;

const USAGE_MSG: &str = "Usage: iseven_api [integer]";

#[tokio::main]
async fn main() {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        eprintln!("{}", USAGE_MSG);
    } else {
        match argv[1].parse::<i32>() {
            Ok(num) => {
                let response = iseven_get(num).await;
                match response {
                    Ok(response) => {
                        println!("Advertisement: {}", response.ad());
                        println!("{} is an {} number", num, if response.is_even() { "even" } else { "odd" })
                    }
                    Err(e) => eprintln!("{}", e)
                }
            }
            Err(_) => eprintln!("Invalid number.\n{}", USAGE_MSG)
        }
    }
}