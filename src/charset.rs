// mochou-p/rspond/src/charset.rs

pub enum Charset {
    Ascii,
    Utf8,
    Utf16,
    Utf32
}

impl Charset {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::Ascii => b"charset=US-ASCII",
            Self::Utf8  => b"charset=UTF-8",
            Self::Utf16 => b"charset=UTF-16",
            Self::Utf32 => b"charset=UTF-32"
        }
    }
}

