use derive_into_dynamo::IntoDynamoItem;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{Error, Result};

use base64::{engine::general_purpose, Engine as _};

pub fn from_b64(b64: &str) -> Result<Boostagram> {
    let json_raw = general_purpose::STANDARD.decode(b64)?;
    serde_json::from_slice(&json_raw).map_err(Error::from)
}

pub fn from_json(json: &str) -> Result<Boostagram> {
    serde_json::from_str(json).map_err(Error::from)
}

#[derive(IntoDynamoItem, Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(rename = "stream")]
    STREAM,
    #[serde(rename = "boost")]
    BOOST,
    #[serde(rename = "lsat")]
    LSAT,
    #[serde(other)]
    Unknown,
}

#[derive(IntoDynamoItem, Serialize, Deserialize, Debug)]
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

    //can't be larger for the lightning network
    pub value_msat: Option<u64>,

    //can't be larger for the lightning network
    pub value_msat_total: Option<u64>,
}

impl Boostagram {
    pub fn to_b64(&self) -> Result<String> {
        let json = serde_json::to_vec(self)?;
        Ok(general_purpose::STANDARD.encode(json))
    }
}

pub struct BoostagramBuilder {
    boostagram: Boostagram,
}

impl BoostagramBuilder {
    pub fn build(self) -> Result<Boostagram> {
        let boost = self.boostagram;

        if boost.podcast.is_none()
            && boost.feed_id.is_none()
            && boost.url.is_none()
            && boost.guid.is_none()
        {
            return Err(Error::BuildingError(
                "either podcast, feed_id, url or guid must be set according to spec".to_string(),
            ));
        }

        Ok(boost)
    }

    pub fn new() -> BoostagramBuilder {
        BoostagramBuilder {
            boostagram: Boostagram {
                podcast: None,
                feed_id: None,
                url: None,
                guid: None,
                episode: None,
                item_id: None,
                episode_guid: None,
                time: None,
                ts: None,
                action: None,
                app_name: None,
                app_version: None,
                boost_link: None,
                message: None,
                name: None,
                pubkey: None,
                seconds_back: None,
                sender_key: None,
                sender_name: None,
                sender_id: None,
                sig_fields: None,
                signature: None,
                speed: None,
                uuid: None,
                value_msat: None,
                value_msat_total: None,
            },
        }
    }

    pub fn podcast(mut self, podcast: String) -> Self {
        self.boostagram.podcast = Some(podcast);
        self
    }

    pub fn feed_id(mut self, feed_id: usize) -> Self {
        self.boostagram.feed_id = Some(feed_id);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.boostagram.url = Some(url);
        self
    }

    pub fn guid(mut self, guid: String) -> Self {
        self.boostagram.guid = Some(guid);
        self
    }

    pub fn episode(mut self, episode: String) -> Self {
        self.boostagram.episode = Some(episode);
        self
    }

    pub fn item_id(mut self, item_id: usize) -> Self {
        self.boostagram.item_id = Some(item_id);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.boostagram.message = Some(message);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.boostagram.name = Some(name);
        self
    }

    pub fn pubkey(mut self, pubkey: String) -> Self {
        self.boostagram.pubkey = Some(pubkey);
        self
    }

    pub fn seconds_back(mut self, seconds_back: usize) -> Self {
        self.boostagram.seconds_back = Some(seconds_back);
        self
    }

    pub fn sender_key(mut self, sender_key: String) -> Self {
        self.boostagram.sender_key = Some(sender_key);
        self
    }

    pub fn sender_name(mut self, sender_name: String) -> Self {
        self.boostagram.sender_name = Some(sender_name);
        self
    }

    pub fn sender_id(mut self, sender_id: String) -> Self {
        self.boostagram.sender_id = Some(sender_id);
        self
    }

    pub fn sig_fields(mut self, sig_fields: String) -> Self {
        self.boostagram.sig_fields = Some(sig_fields);
        self
    }

    pub fn signature(mut self, signature: String) -> Self {
        self.boostagram.signature = Some(signature);
        self
    }

    pub fn speed(mut self, speed: String) -> Self {
        self.boostagram.speed = Some(speed);
        self
    }

    pub fn uuid(mut self, uuid: String) -> Self {
        self.boostagram.uuid = Some(uuid);
        self
    }

    pub fn value_msat(mut self, value_msat: u64) -> Self {
        self.boostagram.value_msat = Some(value_msat);
        self
    }

    pub fn value_msat_total(mut self, value_msat_total: u64) -> Self {
        self.boostagram.value_msat_total = Some(value_msat_total);
        self
    }

    pub fn episode_guid(mut self, episode_guid: String) -> Self {
        self.boostagram.episode_guid = Some(episode_guid);
        self
    }

    pub fn time(mut self, time: String) -> Self {
        self.boostagram.time = Some(time);
        self
    }

    pub fn ts(mut self, ts: usize) -> Self {
        self.boostagram.ts = Some(ts);
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.boostagram.action = Some(action);
        self
    }

    pub fn app_name(mut self, app_name: String) -> Self {
        self.boostagram.app_name = Some(app_name);
        self
    }

    pub fn app_version(mut self, app_version: String) -> Self {
        self.boostagram.app_version = Some(app_version);
        self
    }

    pub fn boost_link(mut self, boost_link: String) -> Self {
        self.boostagram.boost_link = Some(boost_link);
        self
    }
}

impl Default for BoostagramBuilder {
    fn default() -> Self {
        Self::new()
    }
}
