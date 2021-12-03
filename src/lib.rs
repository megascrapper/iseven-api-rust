//! A Rust wrapper for [isEven API](https://isevenapi.xyz/).
//!
//! # Examples
//! A simple commandline app from [`main.rs`](https://github.com/megascrapper/iseven-api-rust/blob/master/src/main.rs):
//! ```no_run
//! use ansi_term::Colour::Red;
//! use human_panic::setup_panic;
//! 
//! use iseven_api::iseven_get;
//! 
//! const USAGE_MSG: &str = "Usage: iseven_api [integer]";
//! 
//! #[tokio::main]
//! async fn main() {
//!     setup_panic!();
//! 
//!     let argv = std::env::args().collect::<Vec<_>>();
//!     if argv.len() != 2 {
//!         eprintln!("{} {}", Red.paint("error:"), USAGE_MSG);
//!     } else {
//!         let num = &argv[1];
//!         match iseven_get(num).await {
//!             Ok(response) => {
//!                 println!("Advertisement: {}", response.ad());
//!                 println!("{} is an {} number", num, if response.iseven() { "even" } else { "odd" })
//!             }
//!             Err(e) => eprintln!("{} {}", Red.paint("error:"), e)
//!         }
//!     }
//! }
//! ```
//! 
//! Cargo.toml:
//! ```toml
//! error-chain = "0.12.4"
//! ansi_term = "0.12.1"
//! human-panic = "1.0.3"
//! iseven_api = "0.4.2"
//! ```

use std::fmt::Display;

use error_chain::error_chain;
use serde::Deserialize;

use crate::iseven::{IsEven, IsEvenError};

pub mod iseven;

const API_URL: &str = "https://api.isevenapi.xyz/api/iseven/";

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        IsEven(IsEvenError);
    }
}

/// sends a GET request to the isEven API for a given number. The return value includes the `bool`
/// value of whether the number is even (`true` indicates an even number) as well as the
/// advertisement.
///
/// # Errors
/// Returns an `Err` if either the API request responded with an error or there is an error in the
/// request or parsing of the response.
///
/// * If the number is outside the range for your [pricing plan](https://isevenapi.xyz/#pricing),
/// it will return the `Number out of range` error as message.
/// * If the input is not a valid number, It returns `Invalid number.` as the message.
/// * If the error is in the request [`reqwest::Error`] is returned.
///
/// # Examples
/// ```
/// use std::error::Error;
/// use iseven_api::iseven_get;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let odd_num = iseven_get(41).await?;
///     let even_num = iseven_get(42).await?;
///
///     assert!(!odd_num.iseven());
///     assert!(even_num.iseven());
///
///     Ok(())
/// }
/// ```
pub async fn iseven_get<T: Display>(number: T) -> crate::Result<IsEven> {
    let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);

    match reqwest::get(&request_url).await?.json().await? {
        IsEvenResponse::Ok(r) => Ok(r),
        IsEvenResponse::Err(e) => Err(e.into())
    }
}

/// A blocking version of [`iseven_get`].
///
/// # Panics
/// This function cannot be executed in an async runtime, as per [`reqwest::blocking`] restriction.
///
/// ``` should_panic
/// # use std::error::Error;
/// # use iseven_api::iseven_get_blocking;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let even_num = iseven_get_blocking(42)?;
///
///     Ok(())
/// }
/// ```
///
/// # Examples
/// ```
/// use std::error::Error;
/// use iseven_api::iseven_get_blocking;
/// fn main() -> Result<(), Box<dyn Error>> {
///     let odd_num = iseven_get_blocking(41)?;
///     let even_num = iseven_get_blocking(42)?;
///
///     assert!(!odd_num.iseven());
///     assert!(even_num.iseven());
///
///     Ok(())
/// }
/// ```
pub fn iseven_get_blocking<T: Display>(number: T) -> crate::Result<IsEven> {
    let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);

    match reqwest::blocking::get(&request_url)?.json()? {
        IsEvenResponse::Ok(r) => Ok(r),
        IsEvenResponse::Err(e) => Err(e.into())
    }
}

/// Enum of response types for serde
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IsEvenResponse {
    Ok(IsEven),
    Err(IsEvenError),
}

#[cfg(test)]
mod tests {
    use crate::{iseven_get, iseven_get_blocking};

    #[tokio::test]
    async fn test_valid_int() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get(a).await.unwrap().iseven());
            assert!(iseven_get(b).await.unwrap().iseven());
        }
    }

    #[tokio::test]
    async fn test_valid_float() {
        let odd_nums = [1.0, 3.0, 5.0, 9.0, 5283.0];
        let even_nums = [0.0, 2.0, 8.0, 10.0, 88888.0];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get(a).await.unwrap().iseven());
            assert!(iseven_get(b).await.unwrap().iseven());
        }
    }

    #[tokio::test]
    async fn test_out_of_range() {
        let nums = [1000000, i32::MAX, -1];
        for &a in nums.iter() {
            assert!(iseven_get(a).await.is_err());
        }
    }

    #[tokio::test]
    async fn test_invalid_input() {
        let values = ["abc", "1.0.0", "hello world", "3.14"];
        for &a in values.iter() {
            assert!(iseven_get(a).await.is_err());
        }
    }

    // blocking tests
    #[test]
    fn test_valid_int_blocking() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get_blocking(a).unwrap().iseven());
            assert!(iseven_get_blocking(b).unwrap().iseven());
        }
    }

    #[test]
    fn test_valid_float_blocking() {
        let odd_nums = [1.0, 3.0, 5.0, 9.0, 5283.0];
        let even_nums = [0.0, 2.0, 8.0, 10.0, 88888.0];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get_blocking(a).unwrap().iseven());
            assert!(iseven_get_blocking(b).unwrap().iseven());
        }
    }

    #[test]
    fn test_out_of_range_blocking() {
        let nums = [1000000, i32::MAX, -1];
        for &a in nums.iter() {
            assert!(iseven_get_blocking(a).is_err());
        }
    }

    #[test]
    fn test_invalid_input_blocking() {
        let values = ["abc", "1.0.0", "hello world", "3.14"];
        for &a in values.iter() {
            assert!(iseven_get_blocking(a).is_err());
        }
    }
}
