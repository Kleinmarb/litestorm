use crate::QueryPairs;
use rustc_hash::FxHashMap;

pub(crate) async fn extract_method_and_path(headers: Vec<&str>) -> Option<(String, String)> {
    match headers.get(0) {
        Some(request_line) => {
            let request_line: Vec<&str> = request_line.split(" ").collect();
            match request_line.as_slice() {
                [method, path, ..] => Some((method.to_string(), path.to_string())),
                _ => None,
            }
        }
        None => None,
    }
}

pub(crate) async fn parse_query_string(path: &str) -> QueryPairs {
    let mut result: QueryPairs = FxHashMap::default();
    if let Some(query) = path.split('?').nth(1) {
        for pair in query.split('&') {
            let mut key_value = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (key_value.next(), key_value.next()) {
                result.insert(key.to_string(), value.to_string());
            }
        }
    }
    result
}

pub(crate) async fn is_http_status_code(response: &str) -> bool {
    response.starts_with("HTTP/1.1 100 Continue")
        || response.starts_with("HTTP/1.1 101 Switching Protocols")
        || response.starts_with("HTTP/1.1 102 Processing")
        || response.starts_with("HTTP/1.1 103 Early Hints")
        || response.starts_with("HTTP/1.1 200 OK")
        || response.starts_with("HTTP/1.1 201 Created")
        || response.starts_with("HTTP/1.1 202 Accepted")
        || response.starts_with("HTTP/1.1 203 Non-Authoritative Information")
        || response.starts_with("HTTP/1.1 204 No Content")
        || response.starts_with("HTTP/1.1 205 Reset Content")
        || response.starts_with("HTTP/1.1 206 Partial Content")
        || response.starts_with("HTTP/1.1 207 Multi-Status")
        || response.starts_with("HTTP/1.1 208 Already Reported")
        || response.starts_with("HTTP/1.1 226 IM Used")
        || response.starts_with("HTTP/1.1 300 Multiple Choices")
        || response.starts_with("HTTP/1.1 301 Moved Permanently")
        || response.starts_with("HTTP/1.1 302 Found")
        || response.starts_with("HTTP/1.1 303 See Other")
        || response.starts_with("HTTP/1.1 304 Not Modified")
        || response.starts_with("HTTP/1.1 305 Use Proxy")
        || response.starts_with("HTTP/1.1 307 Temporary Redirect")
        || response.starts_with("HTTP/1.1 308 Permanent Redirect")
        || response.starts_with("HTTP/1.1 400 Bad Request")
        || response.starts_with("HTTP/1.1 401 Unauthorized")
        || response.starts_with("HTTP/1.1 402 Payment Required")
        || response.starts_with("HTTP/1.1 403 Forbidden")
        || response.starts_with("HTTP/1.1 404 Not Found")
        || response.starts_with("HTTP/1.1 405 Method Not Allowed")
        || response.starts_with("HTTP/1.1 406 Not Acceptable")
        || response.starts_with("HTTP/1.1 407 Proxy Authentication Required")
        || response.starts_with("HTTP/1.1 408 Request Timeout")
        || response.starts_with("HTTP/1.1 409 Conflict")
        || response.starts_with("HTTP/1.1 410 Gone")
        || response.starts_with("HTTP/1.1 411 Length Required")
        || response.starts_with("HTTP/1.1 412 Precondition Failed")
        || response.starts_with("HTTP/1.1 413 Payload Too Large")
        || response.starts_with("HTTP/1.1 414 URI Too Long")
        || response.starts_with("HTTP/1.1 415 Unsupported Media Type")
        || response.starts_with("HTTP/1.1 416 Range Not Satisfiable")
        || response.starts_with("HTTP/1.1 417 Expectation Failed")
        || response.starts_with("HTTP/1.1 418 I'm a teapot")
        || response.starts_with("HTTP/1.1 421 Misdirected Request")
        || response.starts_with("HTTP/1.1 422 Unprocessable Entity")
        || response.starts_with("HTTP/1.1 423 Locked")
        || response.starts_with("HTTP/1.1 424 Failed Dependency")
        || response.starts_with("HTTP/1.1 425 Too Early")
        || response.starts_with("HTTP/1.1 426 Upgrade Required")
        || response.starts_with("HTTP/1.1 428 Precondition Required")
        || response.starts_with("HTTP/1.1 429 Too Many Requests")
        || response.starts_with("HTTP/1.1 431 Request Header Fields Too Large")
        || response.starts_with("HTTP/1.1 451 Unavailable For Legal Reasons")
        || response.starts_with("HTTP/1.0 500 Internal Server Error")
        || response.starts_with("HTTP/2.0 501 Not Implemented")
        || response.starts_with("HTTP/2.0 502 Bad Gateway")
        || response.starts_with("HTTP/2.0 503 Service Unavailable")
        || response.starts_with("HTTP/2.0 504 Gateway Timeout")
        || response.starts_with("HTTP/2.0 505 HTTP Version Not Supported")
        || response.starts_with("HTTP/1.1 506 Variant Also Negotiates")
        || response.starts_with("HTTP/1.1 507 Insufficient Storage")
        || response.starts_with("HTTP/1.1 508 Loop Detected")
        || response.starts_with("HTTP/1.1 510 Not Extended")
        || response.starts_with("HTTP/1.1 511 Network Authentication Required")
}
