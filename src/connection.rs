// mochou-p/rspond/src/connection.rs

pub enum Connection {
    Close,
    KeepAlive
}

impl Connection {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::Close     => b"Connection: close",
            Self::KeepAlive => b"Connection: keep-alive"
        }
    }
}
