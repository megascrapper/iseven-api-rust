//! Contains the response types.

use std::fmt::{Display, Formatter};

use serde::Deserialize;

/// Struct containing the return response from the API.
#[derive(Deserialize, Debug, Clone)]
pub struct IsEven {
    ad: String,
    iseven: bool,
}

impl IsEven {
    /// Returns `true` if the number is even.
    pub fn iseven(&self) -> bool {
        self.iseven
    }

    /// Returns the ad message.
    pub fn ad(&self) -> &str {
        &self.ad
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
