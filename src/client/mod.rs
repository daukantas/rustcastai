use std::io::Read;
use std::fmt::Debug;

use serde_json;
use serde::de::Deserialize;

use curs;
use curs::hyper::header::Authorization;
use curs::{Request, FileUpload, Method, StatusCode};

use super::constants;
use super::response::Response;

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

    pub fn text_request(&self, text: &str) {
        let mut req = Request::new(Method::Post, constants::REQUEST_ENDPOINT);
        req.header(Authorization(format!("Token {}", self.token)));
        req.params(vec![("text", text)]);

        req.send().and_then(|x| {
            Self::parse_response::<Response>(x);
            Ok(52)
        });
    }

    fn parse_response<T: Deserialize + Debug>(mut res: curs::Response) {
        if res.status != StatusCode::Ok {
            // return Err(RecastError::Status(res.status))
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{}", body);

        let body: Result<Body<T>, _> = serde_json::from_str(&body);
        println!("DESERIALIZED {:?}", body);
    }
}
