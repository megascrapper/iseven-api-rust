//! A Rust wrapper for [isEven API](https://isevenapi.xyz/).

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
/// Returns an `Err` if the API request is successful but responded with an error.
/// * If the number is outside the range for your [pricing plan](https://isevenapi.xyz/#pricing),
/// it will return the `Number out of range` error as message.
/// * If the input is not a valid number, It returns `Invalid number.` as the message.
///
/// # Panics
/// If there is an error in the request or parsing of the response.
pub async fn iseven_get<T: Display>(number: T) -> crate::Result<IsEven> {
    let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);

    match reqwest::get(&request_url).await?.json().await? {
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
    use crate::iseven_get;

    #[tokio::test]
    async fn test_valid_int() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get(a).await.unwrap().is_even());
            assert!(iseven_get(b).await.unwrap().is_even());
        }
    }

    #[tokio::test]
    async fn test_valid_float() {
        let odd_nums = [1.0, 3.0, 5.0, 9.0, 5283.0];
        let even_nums = [0.0, 2.0, 8.0, 10.0, 88888.0];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get(a).await.unwrap().is_even());
            assert!(iseven_get(b).await.unwrap().is_even());
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
}
