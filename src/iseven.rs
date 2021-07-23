use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

#[derive(Deserialize, Debug)]
pub struct IsEven {
    ad: String,
    iseven: bool,
}

impl IsEven {
    pub fn is_even(&self) -> bool {
        self.iseven
    }

    pub fn ad(&self) -> &String {
        &self.ad
    }
}


#[derive(Deserialize, Debug)]
pub struct IsEvenError {
    error: String,
}

impl IsEvenError {
    pub fn new(error: String) -> IsEvenError {
        IsEvenError { error }
    }

    pub fn error_message(&self) -> &String {
        &self.error
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum IsEvenResponse {
    Ok(IsEven),
    Err(IsEvenError)
}

impl fmt::Display for IsEvenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

