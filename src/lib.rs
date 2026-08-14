// mochou-p/rspond/src/lib.rs

pub mod prelude {
    pub use super::{
        ResponseBuilder,
        ResponseNeedsHttpVersion,
        ResponseNeedsStatusCode,
        ResponseNeedsHeaders,
        ResponseNeedsBody,
        Response,

        HttpVersion,
        StatusCode,
        Header,

        Connection,
        MimeType,
        Charset
    };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

const CRLF: &'static [u8; 2] = b"\r\n";

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn new() -> ResponseNeedsHttpVersion {
        ResponseNeedsHttpVersion
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum HttpVersion {
    OneOne
}

impl HttpVersion {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::OneOne => b"HTTP/1.1 "
        }
    }
}

pub struct ResponseNeedsHttpVersion;

impl ResponseNeedsHttpVersion {
    pub fn http_version(self, http_version: HttpVersion) -> ResponseNeedsStatusCode {
        ResponseNeedsStatusCode { http_version }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    NotImplemented,
    HttpVersionNotSupported
}

impl StatusCode {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Ok                      => b"200 OK",
            Self::BadRequest              => b"400 Bad Request",
            Self::NotFound                => b"404 Not Found",
            Self::NotImplemented          => b"501 Not Implemented",
            Self::HttpVersionNotSupported => b"505 HTTP Version Not Supported"
        }
    }
}

pub struct ResponseNeedsStatusCode {
    http_version: HttpVersion
}

impl ResponseNeedsStatusCode {
    pub fn status_code(self, status_code: StatusCode) -> ResponseNeedsHeaders {
        let Self { http_version } = self;

        ResponseNeedsHeaders { http_version, status_code }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Header {
    Connection(Connection),
    ContentType(MimeType, Charset),
    ContentLength(usize)
}

impl Header {
    fn build(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);

        match self {
            Self::Connection(connection) => {
                bytes.extend(connection.build());
            },
            Self::ContentType(mime_type, charset) => {
                bytes.extend(mime_type.build());
                bytes.extend(charset  .build());
            },
            Self::ContentLength(count) => {
                bytes.extend(b"Content-Length: ");

                let mut count = *count;

                if count < 10 {
                    bytes.push(b'0' + count as u8);
                } else {
                    let mut div = 1;

                    while div <= count / 10 {
                        div *= 10;
                    }

                    while div > 0 {
                        let digit  = count / div;
                        count     %= div;
                        div       /= 10;

                        bytes.push(b'0' + digit as u8);
                    }
                }
            }
        }

        bytes
    }
}

pub enum Connection {
    Close,
    KeepAlive
}

impl Connection {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Close     => b"Connection: close",
            Self::KeepAlive => b"Connection: keep-alive"
        }
    }
}

pub enum MimeType {
    Html,
    Xhtml
}

impl MimeType {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Html  => b"Content-Type: text/html; ",
            Self::Xhtml => b"Content-Type: application/xhtml+xml; "
        }
    }
}

pub enum Charset {
    Utf8
}

impl Charset {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Utf8 => b"charset=utf-8"
        }
    }
}

pub struct ResponseNeedsHeaders {
    http_version: HttpVersion,
    status_code:  StatusCode
}

impl ResponseNeedsHeaders {
    pub fn no_headers(self) -> ResponseNeedsBody {
        self.headers(vec![])
    }

    pub fn headers(self, headers: Vec<Header>) -> ResponseNeedsBody {
        let Self { http_version, status_code } = self;

        ResponseNeedsBody { http_version, status_code, headers }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ResponseNeedsBody {
    http_version: HttpVersion,
    status_code:  StatusCode,
    headers:      Vec<Header>
}

impl ResponseNeedsBody {
    pub fn empty_body<'a>(self) -> Response<'a> {
        self.body(&[])
    }

    pub fn body<'a>(self, body: &'a [u8]) -> Response<'a> {
        let Self { http_version, status_code, headers } = self;

        Response { http_version, status_code, headers, body }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Response<'a> {
    http_version: HttpVersion,
    status_code:  StatusCode,
    headers:      Vec<Header>,
    body:         &'a [u8]
}

impl Response<'_> {
    pub fn build(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64 * self.headers.len() + self.body.len());

        bytes.extend(self.http_version.build());
        bytes.extend(self.status_code.build());
        bytes.extend(CRLF);

        for header in self.headers.iter() {
            bytes.extend(header.build());
            bytes.extend(CRLF);
        }

        bytes.extend(CRLF);
        bytes.extend(self.body);

        bytes
    }
}

