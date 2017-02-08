use reqwest;
use reqwest::header::Authorization;

use ::client::Client;
use ::error::RecastError;
use ::responses;

pub struct ApiClient<'a> {
    token: &'a str,
    user_slug: &'a str,
    bot_slug: &'a str,
    client: Client<'a>,
}

impl<'a> ApiClient<'a> {
    pub fn new(token: &'a str, user_slug: &'a str, bot_slug: &'a str) -> Result<Self, RecastError> {
        Client::new(token)
            .and_then(|client| Ok(ApiClient {
                client: client,
                token: token,
                user_slug: user_slug,
                bot_slug: bot_slug,
            }))
    }

    pub fn set_language(&mut self, language: &'a str) {
        self.client.set_language(language);
    }

    pub fn text_request(&self, text: &str) -> Result<responses::Request, RecastError> {
        self.client.text_request(text)
    }

    pub fn file_request(&self, file_name: &str) -> Result<responses::Request, RecastError> {
        self.client.file_request(file_name)
    }

    pub fn text_converse(&self, text: &str, conversation_token: Option<&str>) -> Result<responses::Converse, RecastError> {
        self.client.text_converse(text, conversation_token)
    }

    pub fn reset_memory(&self, converation_token: &str) -> Result<reqwest::Response, RecastError> {
        self.client.reset_memory(converation_token)
    }

    pub fn reset_conversation(&self, conversation_token: &str) -> Result<reqwest::Response, RecastError> {
        self.client.reset_conversation(conversation_token)
    }
}
