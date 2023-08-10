mod app;
mod types;
mod utils;

pub mod http;
pub(crate) mod openapi;

pub use simple_async::async_main as entry;
pub use self::app::LiteAPI;

pub(crate) use self::types::{Handler, Routes};
pub(crate) use self::utils::*;

pub use serde_json::to_string as json2string;
use std::fs;
use std::io::Error;

pub fn html2string(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}
