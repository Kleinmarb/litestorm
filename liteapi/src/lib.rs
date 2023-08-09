mod app;
mod types;
mod utils;

pub mod http;
pub(crate) mod openapi;

pub use serde_json::to_string as json2string;

#[macro_export]
macro_rules! html2string {
    ($path:expr) => {{
        use std::fs;
        fs::read_to_string($path)
    }};
}

pub use simple_async::async_main as entry;
pub use self::app::LiteAPI;

pub use self::types::QueryPairs;
pub(crate) use self::types::{Handler, Routes};

pub(crate) use self::utils::*;
