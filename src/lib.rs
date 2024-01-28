//! A Rust wrapper for [isEven API](https://isevenapi.xyz/).
//!
//! Includes the library as well as a simple command line app.
//!
//! # Examples
//! A simple commandline app:
//! ```no_run
//! use std::process::exit;
//! use iseven_api::IsEvenApiBlockingClient;
//!
//! const USAGE_MSG: &str = "Usage: iseven_api [integer]";
//!
//! fn main() {
//!     let argv = std::env::args().collect::<Vec<_>>();
//!     let app_name = &argv[0];
//!     if argv.len() != 2 {
//!         eprintln!("error: {}: {}", app_name, USAGE_MSG);
//!         exit(1);
//!     } else {
//!         let num = &argv[1];
//!         let client = IsEvenApiBlockingClient::new();
//!         match client.get(num) {
//!             Ok(response) => {
//!                 println!("Advertisement: {}", response.ad());
//!                 println!(
//!                     "{} is an {} number",
//!                     num,
//!                     if response.iseven() { "even" } else { "odd" }
//!                 )
//!             }
//!             Err(e) => {
//!                 eprintln!("error: {}: {}", app_name, e);
//!                 exit(1);
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Crate features
//! - **blocking** - Enables [`IsEvenApiBlockingClient`] which is a blocking alternative to [`IsEvenApiClient`]
//! and does not require async runtime. It also enables 'convenience' functions [`is_odd`] and [`is_even`].

#![warn(missing_docs)]

use std::fmt::{Display, Formatter};

use reqwest::{Client, StatusCode};
use serde::Deserialize;

const API_URL: &str = "https://api.isevenapi.xyz/api/iseven/";

/// Checks if a number is even.
///
/// # Panics
///
/// This method will panic if it encounters an error. Use [`IsEvenApiClient`] or [`IsEvenApiBlockingClient`]
/// if you want to handle failures more gracefully.
///
/// As this function internally uses blocking HTTP client, this client must also not be used in an async runtime.
///
///
/// # Examples
/// ```
/// use iseven_api::is_even;
///
/// # fn main() {
/// assert!(is_even(42));
/// # }
#[cfg(feature = "blocking")]
pub fn is_even<T: Display>(number: T) -> bool {
    IsEvenApiBlockingClient::new().get(number).unwrap().iseven()
}

/// Checks if a number is odd.
///
/// # Panics
///
/// This method will panic if it encounters an error. Use [`IsEvenApiClient`] or [`IsEvenApiBlockingClient`]
/// if you want to handle failures more gracefully.
///
/// As this function internally uses blocking HTTP client, this client must also not be used in an async runtime.
///
///
/// # Examples
/// ```
/// use iseven_api::is_odd;
///
/// # fn main() {
/// assert!(is_odd(333));
/// # }
#[cfg(feature = "blocking")]
pub fn is_odd<T: Display>(number: T) -> bool {
    !is_even(number)
}

/// Asynchronous API client for isEven API.
///
/// If you need a blocking client, use [`IsEvenApiBlockingClient`] instead.
///
/// If you're making multiple requests, it's probably a good idea to reuse the client to take advantage of keep-alive
/// connection pooling. ([Learn more](https://docs.rs/reqwest/latest/reqwest/index.html#making-a-get-request))
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use iseven_api::IsEvenApiClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// // Initialise the client
/// let client = IsEvenApiClient::new();
///
/// // Make requests
/// let odd_num = client.get(41).await?;
/// let even_num = client.get(42).await?;
/// assert!(odd_num.isodd());
/// assert!(even_num.iseven());
/// #
/// #   Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct IsEvenApiClient {
    client: Client,
}

impl IsEvenApiClient {
    /// Creates a new instance of [`IsEvenApiClient`] with a default HTTP client.
    pub fn new() -> Self {
        Self::with_client(Client::new())
    }

    /// Creates a new instance of [`IsEvenApiClient`] with a supplied [`reqwest::Client`].
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// sends a GET request to the isEven API for a given number. The return value includes the `bool`
    /// value of whether the number is even (`true` indicates an even number) as well as the
    /// advertisement.
    ///
    /// # Errors
    /// Returns an [`IsEvenApiError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    ///
    /// * If the number is outside the range for your [pricing plan](https://isevenapi.xyz/#pricing),
    /// it will return [`IsEvenApiError::NumberOutOfRange`].
    /// * If the input is not a valid number, it returns [`IsEvenApiError::InvalidNumber`].
    /// * For other API error reponses, it returns [`IsEvenApiError::UnknownErrorResponse`] along with an HTTP status code.
    /// * If the error is in the request [`IsEvenApiError::NetworkError`] is returned.
    pub async fn get<T: Display>(&self, number: T) -> Result<IsEvenApiResponse, IsEvenApiError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);
        let response = self.client.get(request_url).send().await?;
        let status = response.status();
        parse_response(response.json().await?, status)
    }
}

impl Default for IsEvenApiClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Blocking API client for isEven API.
///
///
/// If you're making multiple requests, it's probably a good idea to reuse the client to take advantage of keep-alive
/// connection pooling. ([Learn more](https://docs.rs/reqwest/latest/reqwest/index.html#making-a-get-request))
///
/// As per [`reqwest::blocking`] restriction, this client must not be used in an async runtime. Please use
/// [`IsEvenApiClient`] for that.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use iseven_api::IsEvenApiBlockingClient;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// // Initialise the client
/// let client = IsEvenApiBlockingClient::new();
///
/// // Make requests
/// let odd_num = client.get(41)?;
/// let even_num = client.get(42)?;
/// assert!(odd_num.isodd());
/// assert!(even_num.iseven());
/// #
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "blocking")]
#[derive(Debug, Clone)]
pub struct IsEvenApiBlockingClient {
    client: reqwest::blocking::Client,
}

#[cfg(feature = "blocking")]
impl IsEvenApiBlockingClient {
    /// Creates a new instance of [`IsEvenApiBlockingClient`] with a default HTTP client.
    pub fn new() -> Self {
        Self::with_client(reqwest::blocking::Client::new())
    }

    /// Creates a new instance of [`IsEvenApiBlockingClient`] with a supplied [`reqwest::Client`].
    pub fn with_client(client: reqwest::blocking::Client) -> Self {
        Self { client }
    }

    /// sends a GET request to the isEven API for a given number. The return value includes the `bool`
    /// value of whether the number is even (`true` indicates an even number) as well as the
    /// advertisement.
    ///
    /// # Errors
    /// See [`IsEvenApiClient::get`] for a list of possible errors.
    pub fn get<T: Display>(&self, number: T) -> Result<IsEvenApiResponse, IsEvenApiError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);
        let response = self.client.get(request_url).send()?;
        let status = response.status();
        parse_response(response.json()?, status)
    }
}

#[cfg(feature = "blocking")]
impl Default for IsEvenApiBlockingClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Struct containing the return response from the API.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(thiserror::Error, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// An error type containing errors which can result from the API call.
#[derive(thiserror::Error, Debug)]
pub enum IsEvenApiError {
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

fn parse_response(
    json: IsEvenResponseType,
    status: StatusCode,
) -> Result<IsEvenApiResponse, IsEvenApiError> {
    match json {
        IsEvenResponseType::Ok(r) => Ok(r),
        IsEvenResponseType::Err(e) => match status.as_u16() {
            400 => Err(IsEvenApiError::InvalidNumber(e)),
            401 => Err(IsEvenApiError::NumberOutOfRange(e)),
            _ => Err(IsEvenApiError::UnknownErrorResponse(e, status)),
        },
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
    #[cfg(feature = "blocking")]
    fn test_valid_int_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for (&a, b) in ODD_INTS.iter().zip(EVEN_INTS) {
            assert!(client.get(a).unwrap().isodd());
            assert!(client.get(b).unwrap().iseven());
        }
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn test_out_of_range_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for &a in OUT_OF_RANGE_INTS.iter() {
            assert!(client.get(a).is_err());
        }
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn test_invalid_input_blocking() {
        let client = IsEvenApiBlockingClient::new();
        for &a in INVALID_INPUT.iter() {
            assert!(client.get(a).is_err());
        }
    }
}
