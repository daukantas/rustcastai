use std::collections::HashMap;
use serde_json::{Map, Value};

use super::intent::Intent;

/*
 * As serde does not support deserialization for &str
 * we use String instead
 */
#[derive(Debug, Deserialize)]
pub struct Response {
    uuid: String,
    act: String,
    source: String,
    sentiment: String,
    language: String,
    version: String,
    timestamp: String,
    #[serde(rename="type")]
    sentence_type: String,
    status: i32,
    intents: Vec<Intent>,
    entities: HashMap<String, Vec<Map<String, Value>>>,
}

impl Response {

    pub fn intent(&self) -> Option<&Intent> {
        self.intents.first()
    }

    pub fn all(&self, name: &str) -> Option<&Vec<Map<String, Value>>> {
        self.entities.get(name)
    }

    pub fn get(&self, name: &str) -> Option<&Map<String, Value>> {
        self.entities.get(name).and_then(|e| e.first())
    }
}
