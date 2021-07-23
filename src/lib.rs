use std::fmt::Display;
use num_traits::Num;

use crate::iseven::{IsEven, IsEvenError, IsEvenResponse};

mod iseven;

const API_URL: &str = "https://api.isevenapi.xyz/api/iseven";

pub async fn iseven_get<T: Num + Display>(number: T) -> Result<IsEven, IsEvenError> {
    let request_url = format!("{api_root}/{num}/", api_root = API_URL, num = number);

    let response = reqwest::get(&request_url).await;
    match response {
        Ok(response) => {
            let iseven_instance: Result<IsEvenResponse, reqwest::Error> = response.json().await;
            match iseven_instance {
                Ok(iseven_instance) => {
                    match iseven_instance {
                        IsEvenResponse::Ok(r) => Ok(r),
                        IsEvenResponse::Err(e) => Err(e)
                    }
                }
                Err(e) => {
                    panic!("Could not process response: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Error in request: {}", e);
        }
    }
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
}
