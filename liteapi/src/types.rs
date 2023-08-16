use crate::http::{Response, QueryParams};
use rustc_hash::FxHashMap;

pub(crate) type Routes = FxHashMap<String, (String, Handler)>;
pub(crate) type Handler = fn(QueryParams) -> Response;
pub(crate) type MiddlewareHandler = fn(String);
