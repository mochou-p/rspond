// mochou-p/rspond/src/status_code.rs

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
pub enum StatusCode {
    Informational(Informational),
    Successful(Successful),
    Redirection(Redirection),
    ClientError(ClientError),
    ServerError(ServerError)
}

impl StatusCode {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::Informational(informational) => informational.build(),
            Self::Successful(successful)       =>    successful.build(),
            Self::Redirection(redirection)     =>   redirection.build(),
            Self::ClientError(client_error)    =>  client_error.build(),
            Self::ServerError(server_error)    =>  server_error.build()
        }
    }
}

pub enum Informational {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints
}

impl Informational {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Continue           => b"100 Continue",
            Self::SwitchingProtocols => b"101 Switching Protocols",
            Self::Processing         => b"102 Processing",
            Self::EarlyHints         => b"103 Early Hints"
        }
    }
}

pub enum Successful {
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed
}

impl Successful {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Ok                          => b"200 OK",
            Self::Created                     => b"201 Created",
            Self::Accepted                    => b"202 Accepted",
            Self::NonAuthoritativeInformation => b"203 Non-Authoritative Information",
            Self::NoContent                   => b"204 No Content",
            Self::ResetContent                => b"205 Reset Content",
            Self::PartialContent              => b"206 Partial Content",
            Self::MultiStatus                 => b"207 Multi-Status",
            Self::AlreadyReported             => b"208 Already Reported",
            Self::ImUsed                      => b"226 IM Used"
        }
    }
}

pub enum Redirection {
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    Unused,
    TemporaryRedirect,
    PermanentRedirect
}

impl Redirection {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::MultipleChoices   => b"300 Multiple Choices",
            Self::MovedPermanently  => b"301 Moved Permanently",
            Self::Found             => b"302 Found",
            Self::SeeOther          => b"303 See Other",
            Self::NotModified       => b"304 Not Modified",
            Self::UseProxy          => b"305 Use Proxy",
            Self::Unused            => b"306 unused",
            Self::TemporaryRedirect => b"307 Temporary Redirect",
            Self::PermanentRedirect => b"308 Permanent Redirect"
        }
    }
}

pub enum ClientError {
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
    ContentTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons
}

impl ClientError {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::BadRequest                  => b"400 Bad Request",
            Self::Unauthorized                => b"401 Unauthorized",
            Self::PaymentRequired             => b"402 Payment Required",
            Self::Forbidden                   => b"403 Forbidden",
            Self::NotFound                    => b"404 Not Found",
            Self::MethodNotAllowed            => b"405 Method Not Allowed",
            Self::NotAcceptable               => b"406 Not Acceptable",
            Self::ProxyAuthenticationRequired => b"407 Proxy Authentication Required",
            Self::RequestTimeout              => b"408 Request Timeout",
            Self::Conflict                    => b"409 Conflict",
            Self::Gone                        => b"410 Gone",
            Self::LengthRequired              => b"411 Length Required",
            Self::PreconditionFailed          => b"412 Precondition Failed",
            Self::ContentTooLarge             => b"413 Content Too Large",
            Self::UriTooLong                  => b"414 URI Too Long",
            Self::UnsupportedMediaType        => b"415 Unsupported Media Type",
            Self::RangeNotSatisfiable         => b"416 Range Not Satisfiable",
            Self::ExpectationFailed           => b"417 Expectation Failed",
            Self::ImATeapot                   => b"418 I'm a teapot",
            Self::MisdirectedRequest          => b"421 Misdirected Request",
            Self::UnprocessableContent        => b"422 Unprocessable Content",
            Self::Locked                      => b"423 Locked",
            Self::FailedDependency            => b"424 Failed Dependency",
            Self::TooEarly                    => b"425 Too Early",
            Self::UpgradeRequired             => b"426 Upgrade Required",
            Self::PreconditionRequired        => b"428 Precondition Required",
            Self::TooManyRequests             => b"429 Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => b"431 Request Header Fields Too Large",
            Self::UnavailableForLegalReasons  => b"451 Unavailable For Legal Reasons"
        }
    }
}

pub enum ServerError {
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
    NetworkAuthenticationRequired
}

impl ServerError {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::InternalServerError           => b"500 Internal Server Error",
            Self::NotImplemented                => b"501 Not Implemented",
            Self::BadGateway                    => b"502 Bad Gateway",
            Self::ServiceUnavailable            => b"503 Service Unavailable",
            Self::GatewayTimeout                => b"504 Gateway Timeout",
            Self::HttpVersionNotSupported       => b"505 HTTP Version Not Supported",
            Self::VariantAlsoNegotiates         => b"506 Variant Also Negotiates",
            Self::InsufficientStorage           => b"507 Insufficient Storage",
            Self::LoopDetected                  => b"508 Loop Detected",
            Self::NotExtended                   => b"510 Not Extended",
            Self::NetworkAuthenticationRequired => b"511 Network Authentication Required"
        }
    }
}

