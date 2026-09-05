// mochou-p/rspond/src/header.rs

use super::{Connection, MediaType};


pub enum Header {
    Custom(String, String),

    Connection(Connection),
    ContentLength(usize),
    ContentType(MediaType)
}

impl Header {
    pub(super) fn build(&self) -> Vec<u8> {
        match self {
            Self::Custom(name, value)     => format!("{name}: {value}").as_bytes().to_vec(),
            Self::Connection(connection)  => connection.build().to_vec(),
            Self::ContentType(media_type) => media_type.build().to_vec(),
            Self::ContentLength(count)    => {
                let mut bytes = Vec::with_capacity(64);
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

                bytes
            }
        }
    }
}

