use reqwest;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum RecastError {
    FileError,
    IOError(io::Error),
    Parse(serde_json::Error),
    RequestError(reqwest::Response),
    HTTPClientError(reqwest::Error),
}
