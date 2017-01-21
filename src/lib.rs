#![feature(custom_attribute)]

//! Wrapper around the Recast.AI API
//!
//! ## Overview
//!
//! This crate is a wrapper around the open API of Recast.AI.
//! It allows you to build bots and analyze text and audio files

extern crate curs;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod traits;

/// Representation of the API resources (Action, Intent,...)
pub mod resources;

pub mod responses;

/// Rustcastai Client to perform requests to the Recast.AI API
pub mod client;
pub use client::Client;

/// Rustcastai's error types
pub mod error;

/// Bunch of constants used in the SDK
pub mod constants {
    pub const REQUEST_ENDPOINT: &'static str = "https://api.recast.ai/v2/request";
    pub const CONVERSE_ENDPOINT: &'static str = "https://api.recast.ai/v2/converse";

    pub const ACT_ASSERT: &'static str = "assert";
    pub const ACT_COMMAND: &'static str = "command";
    pub const ACT_WH_QUERY: &'static str = "wh-query";
    pub const ACT_YN_QUERY: &'static str = "yn-query";

    pub const TYPE_ABBREVIATION: &'static str = "abbr:";
    pub const TYPE_ENTITY: &'static str = "enty:";
    pub const TYPE_DESCRIPTION: &'static str = "desc:";
    pub const TYPE_HUMAN: &'static str = "hum:";
    pub const TYPE_LOCATION: &'static str = "loc:";
    pub const TYPE_NUMBER: &'static str = "num:";

    pub const SENTIMENT_VPOSITIVE: &'static str = "vpositive";
    pub const SENTIMENT_POSITIVE: &'static str = "positive";
    pub const SENTIMENT_NEUTRAL: &'static str = "neutral";
    pub const SENTIMENT_NEGATIVE: &'static str = "negative";
    pub const SENTIMENT_VNEGATIVE: &'static str = "vnegative";
}
