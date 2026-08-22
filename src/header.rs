// mochou-p/rspond/src/header.rs

use super::{Connection, MimeType, Charset};


pub enum Header {
    Connection(Connection),
    ContentLength(usize),
    ContentType(MimeType, Charset)
}

impl Header {
    pub(super) fn build(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(64);

        match self {
            Self::Connection(connection) => {
                bytes.extend(connection.build());
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
            },
            Self::ContentType(mime_type, charset) => {
                bytes.extend(mime_type.build());
                bytes.extend(charset  .build());
            }
        }

        bytes
    }
}

