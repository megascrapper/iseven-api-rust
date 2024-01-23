//! A Rust wrapper for [isEven API](https://isevenapi.xyz/).
//!
//! Includes the library as well as a simple command line app.

use std::fmt::{Display, Formatter};

use reqwest::{Client, StatusCode};
use serde::Deserialize;

const API_URL: &str = "https://api.isevenapi.xyz/api/iseven/";

/// Checks if a number is even.
///
/// **Note:** this method will panic if it encounters an error.
pub fn is_even<T: Display>(number: T) -> bool {
    IsEvenApiBlockingClient::new().get(number).unwrap().iseven()
}

/// Checks if a number is odd.
///
/// **Note:** this method will panic if it encounters an error.
pub fn is_odd<T: Display>(number: T) -> bool {
    !is_even(number)
}

/// An error type containing errors which can result from the API call.
#[derive(thiserror::Error, Debug)]
pub enum IsEvenError {
    /// Number out of range for your [pricing plan](https://isevenapi.xyz/#pricing)
    #[error(transparent)]
    NumberOutOfRange(IsEvenApiErrorResponse),
    /// Invalid number specified
    #[error(transparent)]
    InvalidNumber(IsEvenApiErrorResponse),
    /// Unknown error response received, with HTTP status code
    #[error("Server returned status code {1}: {0}")]
    UnknownErrorResponse(IsEvenApiErrorResponse, StatusCode),
    /// Error in making API request
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

/// Enum of response types for serde
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IsEvenResponseType {
    Ok(IsEvenApiResponse),
    Err(IsEvenApiErrorResponse),
}

pub struct IsEvenApiClient {
    client: Client
}

impl IsEvenApiClient {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }

    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// sends a GET request to the isEven API for a given number. The return value includes the `bool`
    /// value of whether the number is even (`true` indicates an even number) as well as the
    /// advertisement.
    ///
    /// # Errors
    /// Returns an [`IsEvenError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    ///
    /// * If the number is outside the range for your [pricing plan](https://isevenapi.xyz/#pricing),
    /// it will return [`IsEvenError::NumberOutOfRange`].
    /// * If the input is not a valid number, it returns [`IsEvenError::InvalidNumber`].
    /// * For other API error reponses, it returns [`IsEvenError::UnknownErrorResponse`] along with an HTTP status code.
    /// * If the error is in the request [`IsEvenError::NetworkError`] is returned.
    ///
    /// # Examples
    /// TODO
    pub async fn get<T: Display>(&self, number: T) -> Result<IsEvenApiResponse, IsEvenError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);
        let response = self.client.get(&request_url).send().await?;
        let status = response.status();
        parse_response(response.json().await?, status)
    }
}

pub struct IsEvenApiBlockingClient {
    client: reqwest::blocking::Client
}

impl IsEvenApiBlockingClient {
    pub fn new() -> Self {
        Self { client: reqwest::blocking::Client::new() }
    }

    pub fn with_client(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    pub fn get<T: Display>(&self, number: T) -> Result<IsEvenApiResponse, IsEvenError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);
        let response = self.client.get(&request_url).send()?;
        let status = response.status();
        parse_response(response.json()?, status)
    }
}

fn parse_response(json: IsEvenResponseType, status: StatusCode) -> Result<IsEvenApiResponse, IsEvenError> {
    match json {
        IsEvenResponseType::Ok(r) => Ok(r),
        IsEvenResponseType::Err(e) => match status.as_u16() {
            400 => Err(IsEvenError::InvalidNumber(e)),
            401 => Err(IsEvenError::NumberOutOfRange(e)),
            _ => Err(IsEvenError::UnknownErrorResponse(e, status))
        },
    }
}

/// Struct containing the return response from the API.
#[derive(Deserialize, Debug, Clone)]
pub struct IsEvenApiResponse {
    ad: String,
    iseven: bool,
}

impl IsEvenApiResponse {
    /// Returns `true` if the number is even.
    pub fn iseven(&self) -> bool {
        self.iseven
    }

    /// Returns the ad message.
    pub fn ad(&self) -> &str {
        &self.ad
    }

    /// Returns `true` if the number is odd.
    pub fn isodd(&self) -> bool {
        !self.iseven()
    }
}

impl Display for IsEvenApiResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.iseven { "even" } else { "odd" })
    }
}

/// Struct containing the error response from the API.
#[derive(thiserror::Error, Deserialize, Debug, Clone)]
#[error("{}", self.error)]
pub struct IsEvenApiErrorResponse {
    error: String,
}

impl IsEvenApiErrorResponse {
    /// Returns the error message.
    pub fn error(&self) -> &str {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const ODD_INTS: [i32; 5] = [1, 3, 5, 9, 5283];
    const EVEN_INTS: [i32; 5] = [0, 2, 8, 10, 88888];
    const OUT_OF_RANGE_INTS: [i32; 3] = [1000000, i32::MAX, -1];
    const INVALID_INPUT: [&str; 4] = ["abc", "1.0.0", "hello world.as_u16()", "3.14"];

    #[tokio::test]
    async fn test_valid_int() {
        let client = IsEvenApiClient::new();
        for (&a, b) in ODD_INTS.iter().zip(EVEN_INTS) {
            assert!(client.get(a).await.unwrap().isodd());
            assert!(client.get(b).await.unwrap().iseven());
        }
    }

    #[tokio::test]
    async fn test_out_of_range() {
        let client = IsEvenApiClient::new();
        for &a in OUT_OF_RANGE_INTS.iter() {
            assert!(client.get(a).await.is_err());
        }
    }

    #[tokio::test]
    async fn test_invalid_input() {
        let client = IsEvenApiClient::new();
        for &a in INVALID_INPUT.iter() {
            assert!(client.get(a).await.is_err());
        }
    }

    // blocking tests
    #[test]
    fn test_valid_int_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for (&a, b) in ODD_INTS.iter().zip(EVEN_INTS) {
            assert!(client.get(a).unwrap().isodd());
            assert!(client.get(b).unwrap().iseven());
        }
    }

    #[test]
    fn test_out_of_range_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for &a in OUT_OF_RANGE_INTS.iter() {
            assert!(client.get(a).is_err());
        }
    }

    #[test]
    fn test_invalid_input_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for &a in INVALID_INPUT.iter() {
            assert!(client.get(a).is_err());
        }
    }
}
