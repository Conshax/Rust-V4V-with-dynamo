use crate::Error;

use super::{Action, Boostagram};

pub struct BoostagramBuilder {
    boostagram: Boostagram,
}

impl BoostagramBuilder {
    pub fn build(self) -> Result<Boostagram, Error> {
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
                sender_name: None,
                sender_id: None,
                signature: None,
                speed: None,
                uuid: None,
                value_msat_total: None,
                reply_address: None,
                reply_custom_key: None,
                reply_custom_value: None,
            },
        }
    }

    pub fn podcast(mut self, podcast: String) -> Self {
        self.boostagram.podcast = Some(podcast);
        self
    }

    pub fn podcast_opt(mut self, podcast: Option<String>) -> Self {
        self.boostagram.podcast = podcast;
        self
    }

    pub fn feed_id(mut self, feed_id: usize) -> Self {
        self.boostagram.feed_id = Some(feed_id);
        self
    }

    pub fn feed_id_opt(mut self, feed_id: Option<usize>) -> Self {
        self.boostagram.feed_id = feed_id;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.boostagram.url = Some(url);
        self
    }

    pub fn url_opt(mut self, url: Option<String>) -> Self {
        self.boostagram.url = url;
        self
    }

    pub fn guid(mut self, guid: String) -> Self {
        self.boostagram.guid = Some(guid);
        self
    }

    pub fn guid_opt(mut self, guid: Option<String>) -> Self {
        self.boostagram.guid = guid;
        self
    }

    pub fn episode(mut self, episode: String) -> Self {
        self.boostagram.episode = Some(episode);
        self
    }

    pub fn episode_opt(mut self, episode: Option<String>) -> Self {
        self.boostagram.episode = episode;
        self
    }

    pub fn item_id(mut self, item_id: usize) -> Self {
        self.boostagram.item_id = Some(item_id);
        self
    }

    pub fn item_id_opt(mut self, item_id: Option<usize>) -> Self {
        self.boostagram.item_id = item_id;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.boostagram.message = Some(message);
        self
    }

    pub fn message_opt(mut self, message: Option<String>) -> Self {
        self.boostagram.message = message;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.boostagram.name = Some(name);
        self
    }

    pub fn name_opt(mut self, name: Option<String>) -> Self {
        self.boostagram.name = name;
        self
    }

    pub fn reply_address(mut self, reply_address: String) -> Self {
        self.boostagram.reply_address = Some(reply_address);
        self
    }

    pub fn reply_address_opt(mut self, reply_address: Option<String>) -> Self {
        self.boostagram.reply_address = reply_address;
        self
    }

    pub fn reply_custom_key(mut self, reply_custom_key: usize) -> Self {
        self.boostagram.reply_custom_key = Some(reply_custom_key);
        self
    }

    pub fn reply_custom_key_opt(mut self, reply_custom_key: Option<usize>) -> Self {
        self.boostagram.reply_custom_key = reply_custom_key;
        self
    }

    pub fn reply_custom_value(mut self, reply_custom_value: String) -> Self {
        self.boostagram.reply_custom_value = Some(reply_custom_value);
        self
    }

    pub fn reply_custom_value_opt(mut self, reply_custom_value: Option<String>) -> Self {
        self.boostagram.reply_custom_value = reply_custom_value;
        self
    }

    pub fn sender_name(mut self, sender_name: String) -> Self {
        self.boostagram.sender_name = Some(sender_name);
        self
    }

    pub fn sender_name_opt(mut self, sender_name: Option<String>) -> Self {
        self.boostagram.sender_name = sender_name;
        self
    }

    pub fn sender_id(mut self, sender_id: String) -> Self {
        self.boostagram.sender_id = Some(sender_id);
        self
    }

    pub fn sender_id_opt(mut self, sender_id: Option<String>) -> Self {
        self.boostagram.sender_id = sender_id;
        self
    }

    pub fn signature(mut self, signature: String) -> Self {
        self.boostagram.signature = Some(signature);
        self
    }

    pub fn signature_opt(mut self, signature: Option<String>) -> Self {
        self.boostagram.signature = signature;
        self
    }

    pub fn speed(mut self, speed: String) -> Self {
        self.boostagram.speed = Some(speed);
        self
    }

    pub fn speed_opt(mut self, speed: Option<String>) -> Self {
        self.boostagram.speed = speed;
        self
    }

    pub fn uuid(mut self, uuid: String) -> Self {
        self.boostagram.uuid = Some(uuid);
        self
    }

    pub fn uuid_opt(mut self, uuid: Option<String>) -> Self {
        self.boostagram.uuid = uuid;
        self
    }

    pub fn value_msat_total(mut self, value_msat_total: u64) -> Self {
        self.boostagram.value_msat_total = Some(value_msat_total);
        self
    }

    pub fn value_msat_total_opt(mut self, value_msat_total: Option<u64>) -> Self {
        self.boostagram.value_msat_total = value_msat_total;
        self
    }

    pub fn episode_guid(mut self, episode_guid: String) -> Self {
        self.boostagram.episode_guid = Some(episode_guid);
        self
    }

    pub fn episode_guid_opt(mut self, episode_guid: Option<String>) -> Self {
        self.boostagram.episode_guid = episode_guid;
        self
    }

    pub fn time(mut self, time: String) -> Self {
        self.boostagram.time = Some(time);
        self
    }

    pub fn time_opt(mut self, time: Option<String>) -> Self {
        self.boostagram.time = time;
        self
    }

    pub fn ts(mut self, ts: usize) -> Self {
        self.boostagram.ts = Some(ts);
        self
    }

    pub fn ts_opt(mut self, ts: Option<usize>) -> Self {
        self.boostagram.ts = ts;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.boostagram.action = Some(action);
        self
    }

    pub fn action_opt(mut self, action: Option<Action>) -> Self {
        self.boostagram.action = action;
        self
    }

    pub fn app_name(mut self, app_name: String) -> Self {
        self.boostagram.app_name = Some(app_name);
        self
    }

    pub fn app_name_opt(mut self, app_name: Option<String>) -> Self {
        self.boostagram.app_name = app_name;
        self
    }

    pub fn app_version(mut self, app_version: String) -> Self {
        self.boostagram.app_version = Some(app_version);
        self
    }

    pub fn app_version_opt(mut self, app_version: Option<String>) -> Self {
        self.boostagram.app_version = app_version;
        self
    }

    pub fn boost_link(mut self, boost_link: String) -> Self {
        self.boostagram.boost_link = Some(boost_link);
        self
    }

    pub fn boost_link_opt(mut self, boost_link: Option<String>) -> Self {
        self.boostagram.boost_link = boost_link;
        self
    }
}

impl Default for BoostagramBuilder {
    fn default() -> Self {
        Self::new()
    }
}
