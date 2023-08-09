#[allow(dead_code)]
pub enum StatusCode  {
    // 1xx Informational
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 2xx Success
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // 3xx Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx Client errors
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    // 5xx Server errors
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl StatusCode  {
    pub fn parse(&self) -> String {
        // Every http status codes string representation
        match self {
            Self::Continue => "HTTP/1.1 100 Continue\r\n\r\nContinue".to_string(),
            Self::SwitchingProtocols => "HTTP/1.1 101 Switching Protocols\r\n\r\nSwitching Protocols".to_string(),
            Self::Processing => "HTTP/1.1 102 Processing\r\n\r\nProcessing".to_string(),
            Self::EarlyHints => "HTTP/1.1 103 Early Hints\r\n\r\nEarly Hints".to_string(),
            Self::OK => "HTTP/1.1 200 OK\r\n\r\nOK".to_string(),
            Self::Created => "HTTP/1.1 201 Created\r\n\r\nCreated".to_string(),
            Self::Accepted => "HTTP/1.1 202 Accepted\r\n\r\nAccepted".to_string(),
            Self::NonAuthoritativeInformation => "HTTP/1.1 203 Non-Authoritative Information\r\n\r\nNon-Authoritative Information".to_string(),
            Self::NoContent => "HTTP/1.1 204 No Content\r\n\r\nNo Content".to_string(),
            Self::ResetContent => "HTTP/1.1 205 Reset Content\r\n\r\nReset Content".to_string(),
            Self::PartialContent => "HTTP/1.1 206 Partial Content\r\n\r\nPartial Content".to_string(),
            Self::MultiStatus => "HTTP/1.1 207 Multi-Status\r\n\r\nMulti-Status".to_string(),
            Self::AlreadyReported => "HTTP/1.1 208 Already Reported\r\n\r\nAlready Reported".to_string(),
            Self::IMUsed => "HTTP/1.1 226 IM Used\r\n\r\nIM Used".to_string(),
            Self::MultipleChoices => "HTTP/1.1 300 Multiple Choices\r\n\r\nMultiple Choices".to_string(),
            Self::MovedPermanently => "HTTP/1.1 301 Moved Permanently\r\n\r\nMoved Permanently".to_string(),
            Self::Found => "HTTP/1.1 302 Found\r\n\r\nFound".to_string(),
            Self::SeeOther => "HTTP/1.1 303 See Other\r\n\r\nSee Other".to_string(),
            Self::NotModified => "HTTP/1.1 304 Not Modified\r\n\r\nNot Modified".to_string(),
            Self::UseProxy => "HTTP/1.1 305 Use Proxy\r\n\r\nUse Proxy".to_string(),
            Self::TemporaryRedirect => "HTTP/1.1 307 Temporary Redirect\r\n\r\nTemporary Redirect".to_string(),
            Self::PermanentRedirect => "HTTP/1.1 308 Permanent Redirect\r\n\r\nPermanent Redirect".to_string(),
            Self::BadRequest => "HTTP/1.1 400 Bad Request\r\n\r\nBad Request".to_string(),
            Self::Unauthorized => "HTTP/1.1 401 Unauthorized\r\n\r\nUnauthorized".to_string(),
            Self::PaymentRequired => "HTTP/1.1 402 Payment Required\r\n\r\nPayment Required".to_string(),
            Self::Forbidden => "HTTP/1.1 403 Forbidden\r\n\r\nForbidden".to_string(),
            Self::NotFound => "HTTP/1.1 404 Not Found\r\n\r\nNot Found".to_string(),
            Self::MethodNotAllowed => "HTTP/1.1 405 Method Not Allowed\r\n\r\nMethod Not Allowed".to_string(),
            Self::NotAcceptable => "HTTP/1.1 406 Not Acceptable\r\n\r\nNot Acceptable".to_string(),
            Self::ProxyAuthenticationRequired => "HTTP/1.1 407 Proxy Authentication Required\r\n\r\nProxy Authentication Required".to_string(),
            Self::RequestTimeout => "HTTP/1.1 408 Request Timeout\r\n\r\nRequest Timeout".to_string(),
            Self::Conflict => "HTTP/1.1 409 Conflict\r\n\r\nConflict".to_string(),
            Self::Gone => "HTTP/1.1 410 Gone\r\n\r\nGone".to_string(),
            Self::LengthRequired => "HTTP/1.1 411 Length Required\r\n\r\nLength Required".to_string(),
            Self::PreconditionFailed => "HTTP/1.1 412 Precondition Failed\r\n\r\nPrecondition Failed".to_string(),
            Self::PayloadTooLarge => "HTTP/1.1 413 Payload Too Large\r\n\r\nPayload Too Large".to_string(),
            Self::URITooLong => "HTTP/1.1 414 URI Too Long\r\n\r\nURI Too Long".to_string(),
            Self::UnsupportedMediaType => "HTTP/1.1 415 Unsupported Media Type\r\n\r\nUnsupported Media Type".to_string(),
            Self::RangeNotSatisfiable => "HTTP/1.1 416 Range Not Satisfiable\r\n\r\nRange Not Satisfiable".to_string(),
            Self::ExpectationFailed => "HTTP/1.1 417 Expectation Failed\r\n\r\nExpectation Failed".to_string(),
            Self::ImATeapot => "HTTP/1.1 418 I'm a teapot\r\n\r\nI'm a teapot".to_string(),
            Self::MisdirectedRequest => "HTTP/1.1 421 Misdirected Request\r\n\r\nMisdirected Request".to_string(),
            Self::UnprocessableEntity => "HTTP/1.1 422 Unprocessable Entity\r\n\r\nUnprocessable Entity".to_string(),
            Self::Locked => "HTTP/1.1 423 Locked\r\n\r\nLocked".to_string(),
            Self::FailedDependency => "HTTP/1.1 424 Failed Dependency\r\n\r\nFailed Dependency".to_string(),
            Self::TooEarly => "HTTP/1.1 425 Too Early\r\n\r\nToo Early".to_string(),
            Self::UpgradeRequired => "HTTP/1.1 426 Upgrade Required\r\n\r\nUpgrade Required".to_string(),
            Self::PreconditionRequired => "HTTP/1.1 428 Precondition Required\r\n\r\nPrecondition Required".to_string(),
            Self::TooManyRequests => "HTTP/1.1 429 Too Many Requests\r\n\r\nToo Many Requests".to_string(),
            Self::RequestHeaderFieldsTooLarge => "HTTP/1.1 431 Request Header Fields Too Large\r\n\r\nRequest Header Fields Too Large".to_string(),
            Self::UnavailableForLegalReasons => "HTTP/1.1 451 Unavailable For Legal Reasons\r\n\r\nUnavailable For Legal Reasons".to_string(),
            Self::InternalServerError => "HTTP/1.0 500 Internal Server Error\r\n\r\nInternal Server Error".to_string(),
            Self::NotImplemented => "HTTP/2.0 501 Not Implemented \r\n\r\nNot Implemented".to_string(),
            Self::BadGateway => "HTTP/2.0 502 Bad Gateway \r\n\r\nBad Gateway".to_string(),
            Self::ServiceUnavailable => "HTTP/2.0 503 Service Unavailable \r\n\r\nService Unavailable".to_string(),
            Self::GatewayTimeout => "HTTP/2.0 504 Gateway Timeout \r\n\r\nGateway Timeout".to_string(),
            Self::HttpVersionNotSupported => "HTTP/2.0 505 HTTP Version Not Supported \r\n\r\nHTTP Version Not Supported".to_string(),
            Self::VariantAlsoNegotiates => "HTTP/1.1 506 Variant Also Negotiates\r\n\r\nVariant Also Negotiates".to_string(),
            Self::InsufficientStorage => "HTTP/1.1 507 Insufficient Storage\r\n\r\nInsufficient Storage".to_string(),
            Self::LoopDetected => "HTTP/1.1 508 Loop Detected\r\n\r\nLoop Detected".to_string(),
            Self::NotExtended => "HTTP/1.1 510 Not Extended\r\n\r\nNot Extended".to_string(),
            Self::NetworkAuthenticationRequired => "HTTP/1.1 511 Network Authentication Required\r\n\r\nNetwork Authentication Required".to_string(),
        }
    }

    pub fn detail(&self, detail: &str) -> String {
        let response = self.parse();
        let separator = "\r\n\r\n";

        if let Some(index) = response.find(separator) {
            let before_separator = &response[..index];

            format!("{}{}{}", before_separator, separator, detail)
        } else {
            response
        }
    }
}

pub enum Response {
    Json(String),
    Html(String),
    Plain(String),
    Css(String),
    Javascript(String),
    Jpeg(String),
    Png(String),
    FormData(String),
}

pub enum Method { // Has no use yet
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Trace,
    Connect,
    Options,
}
