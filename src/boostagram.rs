use derive_into_dynamo::IntoDynamoItem;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

use crate::Error;

use base64::{engine::general_purpose, Engine as _};

pub mod builder;

pub fn from_b64(b64: &str) -> Result<Boostagram, Error> {
    let json_raw = general_purpose::STANDARD.decode(b64)?;
    serde_json::from_slice(&json_raw).map_err(Error::from)
}

pub fn from_json(json: &str) -> Result<Boostagram, Error> {
    serde_json::from_str(json).map_err(Error::from)
}

#[derive(IntoDynamoItem, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    #[serde(rename = "stream")]
    #[dynamo(rename = "STREAM")]
    Stream,
    #[serde(rename = "boost")]
    #[dynamo(rename = "BOOST")]
    Boost,
    #[serde(rename = "lsat")]
    #[dynamo(rename = "LSAT")]
    Lsat,
    #[serde(rename = "auto")]
    #[dynamo(rename = "AUTO")]
    Auto,
    #[serde(other)]
    Unknown,
}

#[derive(IntoDynamoItem, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Boostagram {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podcast: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "feedID")]
    pub feed_id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "itemID")]
    #[serde(deserialize_with = "deserialize_item_id")]
    #[serde(default)]
    pub item_id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode_guid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    //can't be larger for the lightning network
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_msat_total: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_address: Option<String>,

    //TODO why string?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_custom_key: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_custom_value: Option<String>,
}

fn deserialize_item_id<'de, D>(d: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).or(Ok(Option::None))
}

impl Boostagram {
    pub fn to_b64(&self) -> Result<String, Error> {
        let json = serde_json::to_vec(self)?;
        Ok(general_purpose::STANDARD.encode(json))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fountain_invoice() {
        let boost_raw = "eyJhcHBfbmFtZSI6IkZvdW50YWluIiwidmFsdWVfbXNhdF90b3RhbCI6MTAwMDAwLCJuYW1lIjoiQWxieSBUZXN0IFVzZXIgUFVUIiwicG9kY2FzdCI6IlRlc3QgUG9kY2FzdCBBbmNob3IiLCJmZWVkSUQiOjYwMTU2NzEsImFjdGlvbiI6ImJvb3N0Iiwic2VuZGVyX2lkIjoiblNpcTdpZDc4SkFkSDl1WTFwSXkiLCJzZW5kZXJfbmFtZSI6IkBhbHdpbl9jb25zaGF4IiwibWVzc2FnZSI6InRlc3QiLCJpdGVtSUQiOiIxNDkzNDE1NDMwOSIsImJvb3N0X2xpbmsiOiJodHRwczovL2ZvdW50YWluLmZtL2VwaXNvZGUvMTQ5MzQxNTQzMDkiLCJlcGlzb2RlIjoidGhpcyBpcyBhIHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgdmVyeSB2ZXJ5IHZlcnkgbG9uZyBlcGlzb2RlIG5hbWUhISEiLCJ0cyI6MzI5LCJ0aW1lIjoiMDA6MDU6MjkifQ==";

        let result = from_b64(boost_raw);

        dbg!(&result);

        assert!(result.is_ok());

        let boostagram = result.unwrap();

        assert_eq!(boostagram.item_id, None);
    }

    #[test]
    fn test_missing_item_id() {
        let boost_raw = "eyJwb2RjYXN0IjoiQUJDIiwiZmVlZElEIjpudWxsLCJ1cmwiOm51bGwsImd1aWQiOm51bGwsImVwaXNvZGUiOm51bGwsIml0ZW1JRCI6bnVsbCwiZXBpc29kZV9ndWlkIjpudWxsLCJ0aW1lIjpudWxsLCJ0cyI6bnVsbCwiYWN0aW9uIjpudWxsLCJhcHBfbmFtZSI6bnVsbCwiYXBwX3ZlcnNpb24iOm51bGwsImJvb3N0X2xpbmsiOm51bGwsIm1lc3NhZ2UiOm51bGwsIm5hbWUiOm51bGwsInB1YmtleSI6bnVsbCwic2Vjb25kc19iYWNrIjpudWxsLCJzZW5kZXJfa2V5IjpudWxsLCJzZW5kZXJfbmFtZSI6bnVsbCwic2VuZGVyX2lkIjpudWxsLCJzaWdfZmllbGRzIjpudWxsLCJzaWduYXR1cmUiOm51bGwsInNwZWVkIjpudWxsLCJ1dWlkIjpudWxsLCJ2YWx1ZV9tc2F0IjpudWxsLCJ2YWx1ZV9tc2F0X3RvdGFsIjpudWxsfQ==";

        let result = from_b64(boost_raw);

        dbg!(&result);

        assert!(result.is_ok());

        let boostagram = result.unwrap();

        assert_eq!(boostagram.item_id, None);
    }
}
