use std::collections::HashMap;

use reqwest;
use reqwest::header::Authorization;

use ::{constants, responses};
use ::error::RecastError;
use ::traits::ParseResponse;

#[derive(Debug)]
pub struct Client<'a> {
    token: &'a str,
    language: Option<&'a str>,
    client: reqwest::Client,
}

impl<'a> ParseResponse for Client<'a> {}

impl<'a> Client<'a> {
    /// Create a new client
    pub fn new(token: &'a str) -> Result<Self, &str> {
        reqwest::Client::new()
            .map_err(|_| "Failed to create the HTTP Client")
            .and_then(|client| Ok(Client { token: token, language: None, client: client }))
    }

    /// Set the language used to perform text_request, file_request and text_converse
    pub fn set_language(&mut self, language: &'a str) {
        self.language = Some(language);
    }

    /// Call /request endpoint to analyze a text
    pub fn text_request(&self, text: &str) -> Result<responses::Request, RecastError> {
        let mut params = HashMap::new();
        params.insert("text", text);
        if let Some(ref language) = self.language {
            params.insert("language", language);
        }

        self.client.post(constants::REQUEST_ENDPOINT)
            .header(Authorization(format!("Token {}", self.token)))
            .form(&params)
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
            .and_then(|b| Self::parse_response::<responses::Request>(b))
    }

    /// Call /request endpoint to analyze an audio file
    pub fn file_request(&self, file_name: &str) -> Result<responses::Request, RecastError> {
        let file = ::std::fs::File::open(file_name)
            .map_err(|_| RecastError::FileError)?;

        self.client.post(constants::REQUEST_ENDPOINT)
            .header(Authorization(format!("Token {}", self.token)))
            .body(file)
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
            .and_then(|b| Self::parse_response::<responses::Request>(b))
    }

    /// Call the /converse endpoint to interact with a bot
    pub fn text_converse(&self, text: &str, conversation_token: Option<&str>) -> Result<responses::Converse, RecastError> {
        let mut params = HashMap::new();
        params.insert("text", text);
        if let Some(ref language) = self.language { params.insert("language", language); }
        if let Some(token) = conversation_token { params.insert("conversation_token", token); }

        self.client.post(constants::CONVERSE_ENDPOINT)
            .header(Authorization(format!("Token {}", self.token)))
            .form(&params)
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
            .and_then(|b| Self::parse_response::<responses::Converse>(b))
    }
}
