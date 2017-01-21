use reqwest;
use serde_json;
use serde::de::Deserialize;

use ::error::RecastError;

#[derive(Debug, Deserialize)]
pub struct Body<T> {
    results: T,
    message: String,
}

pub trait ParseResponse {
    fn parse_response<T: Deserialize>(res: reqwest::Response) -> Result<T, RecastError> {
        if *res.status() != reqwest::StatusCode::Ok {
            return Err(RecastError::RequestError(res))
        }

        serde_json::from_reader::<reqwest::Response, Body<T>>(res)
            .map_err(|e| RecastError::Parse(e))
            .and_then(|b| Ok(b.results))
    }
}
