use std::env;
use std::io::Read;

use serde_json;
use serde::de::Deserialize;

use curs;
use curs::hyper::header::Authorization;
use curs::{Request, FileUpload, Method, StatusCode};

use super::constants;
use super::response::Response;
use super::error::RecastError;
use super::conversation::Conversation;

#[derive(Debug)]
pub struct Client<'a> {
    pub token: &'a str,
    language: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct Body<T> {
    results: T,
    message: String,
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str) -> Self {
        Client { token: token, language: None }
    }

    pub fn set_language(&mut self, language: &'a str) {
        self.language = Some(language);
    }

    pub fn text_request(&self, text: &str) -> Result<Response, RecastError> {
        let mut req = Request::new(Method::Post, constants::REQUEST_ENDPOINT);
        req.header(Authorization(format!("Token {}", self.token)));
        req.params(vec![("text", text)]);

        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<Response>(x))
    }

    pub fn file_request(&self, file_name: &str) -> Result<Response, RecastError> {
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
            .and_then(|x| Self::parse_response::<Response>(x))
    }

    pub fn text_converse(&self, text: &str, conversation_token: Option<&str>) -> Result<Conversation, RecastError> {
        let mut req = Request::new(Method::Post, constants::CONVERSE_ENDPOINT);
        let mut params = vec![("text", text)];
        if let Some(token) = conversation_token {
            params.push(("conversation_token", token));
        }
        req.header(Authorization(format!("Token {}", self.token)));
        req.params(params);

        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<Conversation>(x))
    }

    fn parse_response<T: Deserialize>(mut res: curs::Response) -> Result<T, RecastError> {
        if res.status != StatusCode::Ok {
            return Err(RecastError::Status(res.status))
        }

        let mut body = String::new();
        res.read_to_string(&mut body).map_err(|e| RecastError::IOError(e))?;

        serde_json::from_str::<Body<T>>(&body)
            .and_then(|b| Ok(b.results))
            .map_err(|e| RecastError::Parse(e))
    }
}
