use std::env;

use curs::hyper::header::Authorization;
use curs::{Request, FileUpload, Method};

use ::{constants, responses};
use ::error::RecastError;
use ::traits::ParseResponse;

#[derive(Debug)]
pub struct Client<'a> {
    token: &'a str,
    language: Option<&'a str>,
}

impl<'a> ParseResponse for Client<'a> {}

impl<'a> Client<'a> {
    /// Create a new client with a Recast.AI token
    pub fn new(token: &'a str) -> Self {
        Client { token: token, language: None }
    }

    /// Set the language used to perform text_request, file_request and text_converse
    pub fn set_language(&mut self, language: &'a str) {
        self.language = Some(language);
    }

    /// Call Recast.AI's API to analyze a text
    pub fn text_request(&self, text: &str) -> Result<responses::Request, RecastError> {
        let mut req = Request::new(Method::Post, constants::REQUEST_ENDPOINT);
        let mut params = vec![("text", text)];
        if let Some(ref language) = self.language {
            params.push(("language", language));
        }

        req.header(Authorization(format!("Token {}", self.token)));
        req.params(params);
        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<responses::Request>(x))
    }

    /// Call Recast.AI's API to analyze an audio file
    pub fn file_request(&self, file_name: &str) -> Result<responses::Request, RecastError> {
        let file = FileUpload {
            path: &env::current_dir().unwrap().join(file_name),
            name: "voice".to_string(),
            mime: None,
        };
        let mut req = Request::new(Method::Post, constants::REQUEST_ENDPOINT);

        req.header(Authorization(format!("Token {}", self.token)));
        req.files(vec![file]);
        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<responses::Request>(x))
    }

    /// Call Recast.AI's API to interact with a bot
    pub fn text_converse(&self, text: &str, conversation_token: Option<&str>) -> Result<responses::Converse, RecastError> {
        let mut req = Request::new(Method::Post, constants::CONVERSE_ENDPOINT);
        let mut params = vec![("text", text)];
        if let Some(token) = conversation_token {
            params.push(("conversation_token", token));
        }
        if let Some(ref language) = self.language {
            params.push(("language", language));
        }

        req.header(Authorization(format!("Token {}", self.token)));
        req.params(params);
        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<responses::Converse>(x))
    }
}
