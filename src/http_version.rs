// mochou-p/rspond/src/http_version.rs

pub enum HttpVersion {
    OneOne
}

impl HttpVersion {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::OneOne => b"HTTP/1.1 "
        }
    }
}

