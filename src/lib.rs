use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub mod boostagram;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json error: {0}")]
    DeserializingError(#[from] serde_json::Error),

    #[error("b64 decoding error: {0}")]
    B64DecodeError(#[from] base64::DecodeError),

    #[error("buidling error: {0}")]
    BuildingError(String),

    #[error("Parameter invalid: {0}")]
    ParameterInvalid(String),
}


//TODO make overflow safe
pub fn calculate_splits(splits: Vec<(u64, bool)>, amt: u64) -> Result<Vec<u64>> {
    let (split_sum, fee_sum) = splits.iter()
        .rfold((0, 0), |(split_sum, fee_sum), (split, is_fee)| {
            if *is_fee {
                (split_sum, fee_sum + split)
            } else {
                (split_sum + split, fee_sum)
            }
        });

    if fee_sum > 100 {
        return Err(Error::ParameterInvalid(String::from("Fee sum greater than 100")));
    }

    let rest_percentage = 100 - fee_sum;

    Ok(splits.into_iter().map( |(split, fee)|
        if fee {
            amt * split / 100
        } else {
            if split_sum == 0 {
                0
            } else {
                (split * rest_percentage * amt) / (split_sum * 100)
            }
        }
    ).collect())
}