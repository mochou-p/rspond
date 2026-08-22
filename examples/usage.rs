// mochou-p/rspond/examples/usage.rs

use rspond::*;


fn main() {
    // 1. without headers or body --------------------------------------

    let simple_response = ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ClientError(ClientError::NotFound))
        .no_headers()
        .empty_body()
        .build();

    line();
    render_special_chars(&simple_response);
    line();

    // -----------------------------------------------------------------

    println!();
    println!();

    // 2. with headers and body ----------------------------------------

    let body = b"<!DOCTYPE html><html><body>hello world</body></html>";

    let advanced_response = ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::Successful(Successful::Ok))
        .headers(vec![
            Header::Connection(Connection::Close),
            Header::ContentType(MediaType::Text(Text::Html, Charset::Utf8)),
            Header::ContentLength(body.len()),
        ])
        .body(body)
        .build();

    line();
    render_special_chars(&advanced_response);
    line();

    // -----------------------------------------------------------------
}

fn line() {
    println!("\x1b[34m{}\x1b[0m", "-".repeat(50));
}

fn render_special_chars(bytes: &[u8]) {
    for (i, byte) in bytes.into_iter().enumerate() {
        match byte {
            b'\n'    => {
                print!("\x1b[36m\\n\x1b[0m");
                if i != bytes.len() - 1 {
                    print!("\n");
                }
            },
            b'\r'    => print!("\x1b[36m\\r\x1b[0m"),
            32..=126 => print!("{}", *byte as char),
            _        =>  todo!("{byte}")
        }
    }

    println!();
}

