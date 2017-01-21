use std::io::Read;

use serde_json;
use serde::de::Deserialize;
use curs::{self, StatusCode};

use super::error::RecastError;

#[derive(Debug, Deserialize)]
struct Body<T> {
    results: T,
    message: String,
}

pub trait ParseResponse {
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
