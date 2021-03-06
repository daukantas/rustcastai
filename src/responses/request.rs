use std::collections::HashMap;
use serde_json::{Map, Value};

use ::constants;
use super::intent::Intent;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub uuid: String,
    pub act: String,
    pub source: String,
    pub sentiment: String,
    pub language: String,
    pub version: String,
    pub timestamp: String,
    #[serde(rename="type")]
    pub sentence_type: String,
    pub status: i32,
    pub intents: Vec<Intent>,
    pub entities: HashMap<String, Vec<Map<String, Value>>>,
}

impl Request {

    pub fn intent(&self) -> Option<&Intent> {
        self.intents.first()
    }

    pub fn all(&self, name: &str) -> Option<&Vec<Map<String, Value>>> {
        self.entities.get(name)
    }

    pub fn get(&self, name: &str) -> Option<&Map<String, Value>> {
        self.entities.get(name).and_then(|e| e.first())
    }

    /*
     * Act helper
     */

    pub fn is_assert(&self) -> bool {
        self.act.eq(constants::ACT_ASSERT)
    }

    pub fn is_command(&self) -> bool {
        self.act.eq(constants::ACT_COMMAND)
    }

    pub fn is_wh_query(&self) -> bool {
        self.act.eq(constants::ACT_WH_QUERY)
    }

    pub fn is_yn_query(&self) -> bool {
        self.act.eq(constants::ACT_YN_QUERY)
    }

    /*
     * Type helper
     */

    pub fn is_abbreviation(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_ABBREVIATION)
    }

    pub fn is_entity(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_ENTITY)
    }

    pub fn is_description(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_DESCRIPTION)
    }

    pub fn is_human(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_HUMAN)
    }

    pub fn is_location(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_LOCATION)
    }

    pub fn is_number(&self) -> bool {
        self.sentence_type.starts_with(constants::TYPE_NUMBER)
    }

    /*
     * Sentiment helper
     */

    pub fn is_vpositive(&self) -> bool {
        self.sentiment.eq(constants::SENTIMENT_VPOSITIVE)
    }

    pub fn is_positive(&self) -> bool {
        self.sentiment.eq(constants::SENTIMENT_POSITIVE)
    }

    pub fn is_neutral(&self) -> bool {
        self.sentiment.eq(constants::SENTIMENT_NEUTRAL)
    }

    pub fn is_negative(&self) -> bool {
        self.sentiment.eq(constants::SENTIMENT_NEGATIVE)
    }

    pub fn is_vnegative(&self) -> bool {
        self.sentiment.eq(constants::SENTIMENT_VNEGATIVE)
    }
}
