mod iseven;

use num_traits::Num;
use std::fmt::Display;

use crate::iseven::{IsEven, IsEvenError};
const API_URL: &str = "https://api.isevenapi.xyz/api/iseven";

pub async fn iseven_get<T: Num + Display>(number: T) -> Result<IsEven, IsEvenError> {
    let request_url = format!("{api_root}/{num}/", api_root=API_URL, num=number);

    let response = reqwest::get(&request_url).await;

    match response {
        Ok(response) => {
            let iseven_instance: Result<IsEven, reqwest::Error>= response.json().await;
            match iseven_instance {
                Ok(iseven_instance) => {
                    Ok(iseven_instance)
                },
                Err(e) => {
                    Err(IsEvenError::new("Invalid number".to_string()))
                }
            }
        },
        Err(_) => {
            Err(IsEvenError::new("Error on request".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iseven_get;

    #[tokio::test]
    async fn test_valid_nums() {
        let odd_nums = [1, 3, 5, 9, 5283];
        let even_nums = [0, 2, 8, 10, 88888];
        for (&a, b) in odd_nums.iter().zip(even_nums) {
            assert!(!iseven_get(a).await.unwrap().is_even());
            assert!(iseven_get(b).await.unwrap().is_even());
        }
    }

    #[tokio::test]
    async fn test_out_of_range() {
        let nums = [1000000, i32::MAX, -1];
        for &a in nums.iter() {
            println!("{:?}", iseven_get(a).await);
        }
    }
}
