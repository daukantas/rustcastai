use curs;
use std::io::Read;
use curs::hyper::header::Authorization;
use curs::{Request, FileUpload, Method, StatusCode};

#[derive(Debug)]
pub struct Client<'a> {
    token: &'a str,
    language: Option<&'a str>,
}

struct Body<'a, T> {
    results: T,
    message: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(token: &'a str) -> Self {
        Client { token: token, language: None }
    }

    pub fn set_language(&mut self, language: &'a str) {
        self.language = Some(language);
    }

    pub fn text_converse(&self, text: &str) {
        let mut req = Request::new(Method::Post, "https://api.recast.ai");
        req.header(Authorization(format!("Token {}", self.token)));
        req.params(vec![("text", text)]);

        req.send().and_then(|x| Ok(57));
    }

    fn parse_response(mut res: curs::Response) {
        if res.status != StatusCode::Ok {
            // return Err(RecastError::Status(res.status))
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("{}", body);
    }
}
