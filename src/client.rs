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
    pub client: reqwest::Client,
}

impl<'a> ParseResponse for Client<'a> {}

impl<'a> Client<'a> {
    /// Create a new client
    /// 
    /// The token argument is your bot's request access token.
    /// You can find this token in your bot's settings on Recast.AI
    ///
    /// # Example
    /// ```
    /// let client = Client::new("YOUR_REQUEST_ACCESS_TOKEN");
    /// ```
    pub fn new(token: &'a str) -> Result<Self, RecastError> {
        reqwest::Client::new()
            .map_err(|_| RecastError::Error("Failed to create the HTTP Client".to_string()))
            .and_then(|client| Ok(Client { token: token, language: None, client: client }))
    }

    /// Set the language used to perform text_request, file_request and text_converse
    ///
    /// The language as to be in the form of an isocode, like "en" for english
    /// For now, Recast.AI supports english and french
    ///
    /// # Example
    ///
    /// ```
    /// client.set_language("en");
    /// ```
    pub fn set_language(&mut self, language: &'a str) {
        self.language = Some(language);
    }

    /*
     * Request endpoint
     */

    /// Call /request endpoint to analyze a text
    ///
    /// This method will make an HTTP request on the /request endpoint of the Recast.AI API
    /// and will give you back the intent and metadata detected in your input.
    ///
    /// # Example
    /// ```
    /// client.text_request("Hello, world.");
    /// ```
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
    ///
    /// This method will make an HTTP request on the /request endpoint of the Recast.AI API
    /// and will give you back the intent and metadata detected in the content of your audio file.
    ///
    /// # Example
    /// ```
    /// client.file_request("test.wav");
    /// ```
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

    /*
     * Converse Endpoint
     */

    /// Call the /converse endpoint to interact with a bot
    ///
    /// This method will make an HTTP request on the /converse endpoint of the Recast.AI API 
    /// and will give you back the action and metadata detected, along with the replies of your bot
    ///
    /// # Example
    /// ```
    /// client.text_converse("Hello, what can you do?");
    /// ```
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

    /// Reset the memory of your bot for a specific conversation.
    ///
    /// # Example
    /// ```
    /// client.text_converse("I want to change my destination")
    ///     .and_then(|res| client.reset_memory(&res.conversation_token));
    /// ```
    pub fn reset_memory(&self, conversation_token: &str) -> Result<reqwest::Response, RecastError> {
        let mut params = HashMap::new();
        params.insert("conversation_token", conversation_token);

        self.client.request(reqwest::Method::Put, constants::CONVERSE_ENDPOINT)
            .header(Authorization(format!("Token {}", self.token)))
            .form(&params)
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
    }

    /// Reset the entire conversation: both memory and flow.
    ///
    /// # Example
    /// ```
    /// client.text_converse("Can we start all over again?")
    ///     .and_then(|res| client.reset_conversation(&res.conversation_token));
    /// ```
    pub fn reset_conversation(&self, conversation_token: &str) -> Result<reqwest::Response, RecastError> {
        let mut params = HashMap::new();
        params.insert("conversation_token", conversation_token);

        self.client.request(reqwest::Method::Delete, constants::CONVERSE_ENDPOINT)
            .header(Authorization(format!("Token {}", self.token)))
            .form(&params)
            .send()
            .map_err(|e| RecastError::HTTPClientError(e))
    }
}
