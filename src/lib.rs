//! A Rust wrapper for [isEven API](https://isevenapi.xyz/).
//!
//! Includes the library as well as a simple command line app.

use std::fmt::{Display, Formatter};

use serde::Deserialize;

const API_URL: &str = "https://api.isevenapi.xyz/api/iseven/";

/// Checks if a number is even.
///
/// **Note:** this method will panic if it encounters an error.
pub fn is_even<T: Display>(number: T) -> bool {
    IsEven::get_blocking(number).unwrap().iseven()
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
    /// Error in making API request
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error(transparent)]
    ErrorResponse(#[from] ErrorResponse),
}

/// Enum of response types for serde
#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IsEvenResponse {
    Ok(IsEven),
    Err(ErrorResponse),
}

/// Struct containing the return response from the API.
#[derive(Deserialize, Debug, Clone)]
pub struct IsEven {
    ad: String,
    iseven: bool,
}

impl IsEven {
    /// sends a GET request to the isEven API for a given number. The return value includes the `bool`
    /// value of whether the number is even (`true` indicates an even number) as well as the
    /// advertisement.
    ///
    /// If you are planning on making multiple requests, it is best to use [`Self::with_client()`]
    /// instead and reuse the client, taking advantage of keep-alive connection pooling.
    /// ([Learn more](https://docs.rs/reqwest/0.11.10/reqwest/index.html#making-a-get-request))
    ///
    /// # Errors
    /// Returns an [`IsEvenError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    ///
    /// * If the number is outside the range for your [pricing plan](https://isevenapi.xyz/#pricing),
    /// it will return the `Number out of range` error as message.
    /// * If the input is not a valid number, It returns `Invalid number.` as the message.
    /// * If the error is in the request [`reqwest::Error`] is returned.
    ///
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// use iseven_api::IsEven;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// let odd_num = IsEven::get(41).await?;
    /// let even_num = IsEven::get(42).await?;
    ///
    /// assert!(!odd_num.iseven());
    /// assert!(even_num.iseven());
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn get<T: Display>(number: T) -> Result<IsEven, IsEvenError> {
        Self::with_client(number, reqwest::Client::new()).await
    }

    /// sends a GET request to the isEven API for a given number, using a supplied [`reqwest::Client`].
    pub async fn with_client<T: Display>(
        number: T,
        client: reqwest::Client,
    ) -> Result<IsEven, IsEvenError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);

        match client.get(&request_url).send().await?.json().await? {
            IsEvenResponse::Ok(r) => Ok(r),
            IsEvenResponse::Err(e) => Err(e.into()),
        }
    }

    /// A blocking version of [`Self::get()`].
    ///
    /// If you are planning on making multiple requests, it is best to use [`Self::with_client_blocking()`]
    /// instead and reuse the client, taking advantage of keep-alive connection pooling.
    /// ([Learn more](https://docs.rs/reqwest/0.11.10/reqwest/blocking/index.html#making-a-get-request))
    ///
    /// # Panics
    /// This function cannot be executed in an async runtime, as per [`reqwest::blocking`] restriction.
    ///
    /// ``` should_panic
    /// use std::error::Error;
    /// use iseven_api::IsEven;
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let even_num = IsEven::get_blocking(42)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// use iseven_api::IsEven;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let odd_num = IsEven::get_blocking(41)?;
    /// let even_num = IsEven::get_blocking(42)?;
    ///
    /// assert!(!odd_num.iseven());
    /// assert!(even_num.iseven());
    /// #
    /// #    Ok(())
    /// # }
    /// ```
    pub fn get_blocking<T: Display>(number: T) -> Result<IsEven, IsEvenError> {
        Self::with_client_blocking(number, reqwest::blocking::Client::new())
    }

    /// sends a blocking GET request to the isEven API for a given number, using a supplied [`reqwest::blocking::Client`].
    pub fn with_client_blocking<T: Display>(
        number: T,
        client: reqwest::blocking::Client,
    ) -> Result<IsEven, IsEvenError> {
        let request_url = format!("{api_url}{num}", api_url = API_URL, num = number);

        match client.get(&request_url).send()?.json()? {
            IsEvenResponse::Ok(r) => Ok(r),
            IsEvenResponse::Err(e) => Err(e.into()),
        }
    }

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

impl Display for IsEven {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.iseven { "even" } else { "odd" })
    }
}

/// Struct containing the error response from the API.
#[derive(thiserror::Error, Deserialize, Debug, Clone)]
#[error("{}", self.error)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    /// Returns the error message.
    pub fn error(&self) -> &str {
        &self.error
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_valid_int() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!IsEven::get(a).await.unwrap().iseven());
            assert!(IsEven::get(b).await.unwrap().iseven());
        }
    }

    #[tokio::test]
    async fn test_valid_float() {
        let odd_nums = [1.0, 3.0, 5.0, 9.0, 5283.0];
        let even_nums = [0.0, 2.0, 8.0, 10.0, 88888.0];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!IsEven::get(a).await.unwrap().iseven());
            assert!(IsEven::get(b).await.unwrap().iseven());
        }
    }

    #[tokio::test]
    async fn test_out_of_range() {
        let nums = [1000000, i32::MAX, -1];
        for &a in nums.iter() {
            assert!(IsEven::get(a).await.is_err());
        }
    }

    #[tokio::test]
    async fn test_invalid_input() {
        let values = ["abc", "1.0.0", "hello world", "3.14"];
        for &a in values.iter() {
            assert!(IsEven::get(a).await.is_err());
        }
    }

    // blocking tests
    #[test]
    fn test_valid_int_blocking() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!IsEven::get_blocking(a).unwrap().iseven());
            assert!(IsEven::get_blocking(b).unwrap().iseven());
        }
    }

    #[test]
    fn test_valid_float_blocking() {
        let odd_nums = [1.0, 3.0, 5.0, 9.0, 5283.0];
        let even_nums = [0.0, 2.0, 8.0, 10.0, 88888.0];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!IsEven::get_blocking(a).unwrap().iseven());
            assert!(IsEven::get_blocking(b).unwrap().iseven());
        }
    }

    #[test]
    fn test_out_of_range_blocking() {
        let nums = [1000000, i32::MAX, -1];
        for &a in nums.iter() {
            assert!(IsEven::get_blocking(a).is_err());
        }
    }

    #[test]
    fn test_invalid_input_blocking() {
        let values = ["abc", "1.0.0", "hello world", "3.14"];
        for &a in values.iter() {
            assert!(IsEven::get_blocking(a).is_err());
        }
    }
}
