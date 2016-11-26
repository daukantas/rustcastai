#![feature(proc_macro)]
#![feature(custom_attribute)]

extern crate curs;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod client;
pub mod response;
pub mod intent;
