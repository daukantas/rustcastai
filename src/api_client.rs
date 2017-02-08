use reqwest;
use reqwest::header::Authorization;

use ::client::Client;
use ::error::RecastError;
use ::{responses, constants};
use ::traits::ParseResponse;
use ::resources::{Intent};

#[derive(Debug)]
pub struct ApiClient<'a> {
    token: &'a str,
    user_slug: &'a str,
    bot_slug: &'a str,
    r_client: Client<'a>,
}

impl<'a> ParseResponse for ApiClient<'a> {}

impl<'a> ApiClient<'a> {
    pub fn new(token: &'a str, user_slug: &'a str, bot_slug: &'a str) -> Result<Self, RecastError> {
        Client::new(token)
            .and_then(|client| Ok(ApiClient {
                r_client: client,
                token: token,
                user_slug: user_slug,
                bot_slug: bot_slug,
            }))
    }

    pub fn get_intents(&self) -> Result<Vec<Intent>, RecastError> {
        let url = format!("{}/users/{}/bots/{}/intents", constants::RECAST_ENDPOINT, self.user_slug, self.bot_slug);
        self.r_client.client.get(&url)
            .header(Authorization(format!("Token {}", self.token)))
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
            .and_then(|b| Self::parse_response::<Vec<Intent>>(b))
    }

    /*
     * Wrapper around the Client's methods
     */

    pub fn set_language(&mut self, language: &'a str) {
        self.r_client.set_language(language);
    }

    pub fn text_request(&self, text: &str) -> Result<responses::Request, RecastError> {
        self.r_client.text_request(text)
    }

    pub fn file_request(&self, file_name: &str) -> Result<responses::Request, RecastError> {
        self.r_client.file_request(file_name)
    }

    pub fn text_converse(&self, text: &str, conversation_token: Option<&str>) -> Result<responses::Converse, RecastError> {
        self.r_client.text_converse(text, conversation_token)
    }

    pub fn reset_memory(&self, converation_token: &str) -> Result<reqwest::Response, RecastError> {
        self.r_client.reset_memory(converation_token)
    }

    pub fn reset_conversation(&self, conversation_token: &str) -> Result<reqwest::Response, RecastError> {
        self.r_client.reset_conversation(conversation_token)
    }
}
