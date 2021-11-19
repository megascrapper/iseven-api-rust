//! Contains the response types.

use std::fmt;
use std::fmt::Formatter;

use serde::Deserialize;

/// Struct containing the return response from the API.
#[derive(Deserialize, Debug, Clone)]
pub struct IsEven {
    ad: String,
    iseven: bool,
}

impl IsEven {
    /// Returns `true` if the number is even.
    pub fn is_even(&self) -> bool {
        self.iseven
    }

    /// Returns the ad message.
    pub fn ad(&self) -> &String {
        &self.ad
    }
}


/// Struct containing the error response from the API.
#[derive(thiserror::Error, Deserialize, Debug, Clone)]
pub struct IsEvenError {
    error: String,
}

impl IsEvenError {
    /// Returns the error message.
    pub fn error_message(&self) -> &String {
        &self.error
    }
}

impl fmt::Display for IsEvenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

