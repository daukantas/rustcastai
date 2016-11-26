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
