use crate::http::Response;
use rustc_hash::FxHashMap;

pub type QueryPairs = FxHashMap<String, String>;
pub(crate) type Routes = FxHashMap<String, (String, Handler)>;
pub(crate) type Handler = fn(QueryPairs) -> Response;
