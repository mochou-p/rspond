// mochou-p/rspond/src/charset.rs

pub enum Charset {
    Utf8
}

impl Charset {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::Utf8 => b"charset=utf-8"
        }
    }
}

