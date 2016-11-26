use serde_json::{Map, Value};
use std::collections::HashMap;

use super::intent::Intent;
use super::action::Action;

#[derive(Debug, Deserialize)]
pub struct Conversation {
    uuid: String,
    source: String,
    replies: Vec<String>,
    action: Option<Action>,
    next_actions: Vec<Action>,
    memory: HashMap<String, Map<String, Value>>,
    entities: HashMap<String, Vec<Map<String, Value>>>,
    intents: Vec<Intent>,
    conversation_token: String,
    language: String,
    timestamp: String,
    status: i32,
    version: String,
}

impl Conversation {

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
