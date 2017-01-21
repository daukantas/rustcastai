use reqwest;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum RecastError {
    FileError,
    Error(String),
    IOError(io::Error),
    Parse(serde_json::Error),
    RequestError(reqwest::Response),
    HTTPClientError(reqwest::Error),
}
