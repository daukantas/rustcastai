use std::io::Read;

use serde_json;
use serde::de::Deserialize;

use curs;
use curs::hyper::header::Authorization;
use curs::{Request, FileUpload, Method, StatusCode};

use super::constants;
use super::response::Response;
use super::error::RecastError;

#[derive(Debug)]
pub struct Client<'a> {
    token: &'a str,
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

    pub fn text_request(&self, text: &str) -> Result<Response, RecastError>{
        let mut req = Request::new(Method::Post, constants::REQUEST_ENDPOINT);
        req.header(Authorization(format!("Token {}", self.token)));
        req.params(vec![("text", text)]);

        req.send()
            .map_err(|e| RecastError::Request(e))
            .and_then(|x| Self::parse_response::<Response>(x))
    }

    fn parse_response<T: Deserialize>(mut res: curs::Response) -> Result<T, RecastError>{
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
