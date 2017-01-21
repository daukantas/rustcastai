use reqwest;
use std::io;
use serde_json;

/// [unstable] Error types for rustcastai. Likely to change, place for improvments
#[derive(Debug)]
pub enum RecastError {
    /// Invalid file provided
    FileError,
    /// Could not create an instance of rustcastai::Client
    Error(String),
    /// Error while reading an HTTP response's body
    IOError(io::Error),
    /// Error while parsing an HTTP response body
    Parse(serde_json::Error),
    /// The request didn't returned a 2xx
    RequestError(reqwest::Response),
    /// Error while sending the request
    HTTPClientError(reqwest::Error),
}
