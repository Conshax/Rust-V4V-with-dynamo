use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;

use base64::{Engine as _, engine::general_purpose};

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json error: {0}")]
    Deserializing(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Deserializing(err.to_string())
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::Deserializing(err.to_string())
    }
}

pub fn from_b64(b64: &str) -> Result<Boostagram, Error> {
    let json_raw = general_purpose::STANDARD.decode(b64)?;
    serde_json::from_slice(&json_raw).map_err(Error::from)
}

pub fn from_json(json: &str) -> Result<Boostagram, Error> {
    serde_json::from_str(json).map_err(Error::from)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action{
    #[serde(rename = "stream")]
    STREAM,
    #[serde(rename = "boost")]
    BOOST,
    #[serde(rename = "lsat")]
    LSAT,
    #[serde(other)]
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boostagram {
    pub podcast: Option<String>,

    #[serde(rename = "feedID")]
    pub feed_id: Option<usize>,

    pub url: Option<String>,

    pub guid: Option<String>,

    pub episode: Option<String>,

    #[serde(rename = "itemID")]
    pub item_id: Option<usize>,

    pub episode_guid: Option<String>,

    pub time: Option<String>,

    pub ts: Option<usize>,

    pub action: Option<Action>,

    pub app_name: Option<String>,

    pub app_version: Option<String>,

    pub boost_link: Option<String>,

    pub message: Option<String>,

    pub name: Option<String>,

    pub pubkey: Option<String>,

    pub seconds_back: Option<usize>,

    pub sender_key: Option<String>,

    pub sender_name: Option<String>,

    pub sender_id: Option<String>,

    pub sig_fields: Option<String>,

    pub signature: Option<String>,

    pub speed: Option<String>,

    pub uuid: Option<String>,

    pub value_msat: Option<usize>,

    pub value_msat_total: Option<usize>,

}
