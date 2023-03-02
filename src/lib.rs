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
}