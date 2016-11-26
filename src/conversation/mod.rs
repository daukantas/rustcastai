use serde_json::{Map, Value};
use std::collections::HashMap;

use curs;
use curs::{Request, Method};
use curs::hyper::header::Authorization;

use super::constants;
use super::intent::Intent;
use super::action::Action;
use super::error::RecastError;

#[derive(Debug, Deserialize)]
pub struct Conversation {
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

    /*
     * Static method
     */

    pub fn reset_memory(token: &str, conversation_token: &str) -> Result<curs::Response, RecastError> {
        let mut req = Request::new(Method::Put, constants::CONVERSE_ENDPOINT);
        req.header(Authorization(format!("Token {}", token)));
        req.params(vec![("conversation_token", conversation_token)]);

        req.send().map_err(|e| RecastError::Request(e))
    }

    pub fn reset_conversation(token: &str, conversation_token: &str) -> Result<curs::Response, RecastError> {
        let mut req = Request::new(Method::Delete, constants::CONVERSE_ENDPOINT);
        req.header(Authorization(format!("Token {}", token)));
        req.params(vec![("conversation_token", conversation_token)]);

        req.send().map_err(|e| RecastError::Request(e))
    }
}
