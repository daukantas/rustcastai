use std::collections::HashMap;

use serde_json::{Map, Value};

use super::action::Action;
use super::intent::Intent;

#[derive(Debug, Deserialize)]
pub struct Converse {
    pub uuid: String,
    pub source: String,
    pub replies: Vec<String>,
    pub action: Option<Action>,
    pub next_actions: Vec<Action>,
    pub memory: HashMap<String, Map<String, Value>>,
    pub entities: HashMap<String, Vec<Map<String, Value>>>,
    pub intents: Vec<Intent>,
    pub conversation_token: String,
    pub language: String,
    pub timestamp: String,
    pub status: i32,
    pub version: String,
}

impl Converse {

    pub fn replies(&self) -> Option<&str> {
        self.replies.first().map(|s| s.as_str())
    }

    pub fn next_actions(&self) -> Option<&Action> {
        self.next_actions.first()
    }

    pub fn joined_replies(&self, sep: &str) -> String {
        self.replies.join(sep)
    }

    pub fn get_memory(&self, key: &str) -> Option<&Map<String, Value>> {
        self.memory.get(key)
    }
}
