use serde::Deserialize;

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

