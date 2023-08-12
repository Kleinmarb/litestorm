mod app;
mod types;
mod utils;

pub use simple_async::async_main as entry;
pub use futures_executor::block_on as execute;
pub use self::app::LiteAPI;
pub mod http;

pub(crate) use self::types::{Handler, Routes};
pub(crate) use self::utils::*;
pub(crate) mod openapi;

pub use serde_json::json;
pub use serde_json::to_string as json2string;

use std::fs;
use std::io::Error;

pub fn html2string(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}
