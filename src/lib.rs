// mochou-p/rspond/src/lib.rs

mod http_version;
mod status_code;
mod header;
mod connection;
mod media_type;
mod charset;

pub use {
    http_version::*,
    status_code::*,
    header::*,
    connection::*,
    media_type::*,
    charset::*
};


pub struct ResponseBuilder;

impl ResponseBuilder {
    pub fn new() -> ResponseNeedsHttpVersion {
        ResponseNeedsHttpVersion
    }
}

pub struct ResponseNeedsHttpVersion;

impl ResponseNeedsHttpVersion {
    pub fn http_version(self, http_version: HttpVersion) -> ResponseNeedsStatusCode {
        ResponseNeedsStatusCode { http_version }
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

pub struct Response<'a> {
    http_version: HttpVersion,
    status_code:  StatusCode,
    headers:      Vec<Header>,
    body:         &'a [u8]
}

impl Response<'_> {
    pub fn build(&self) -> Vec<u8> {
        const CRLF: [u8; 2] = *b"\r\n";

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

