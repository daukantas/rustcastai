use curs;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum RecastError {
    Request(curs::CursError),
    Parse(serde_json::Error),
    Status(curs::StatusCode),
    IOError(io::Error),
}
