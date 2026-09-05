// mochou-p/rspond/src/media_type.rs

use super::Charset;


// https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
pub enum MediaType {
    Application(Application),
    Audio(Audio),
    Image(Image),
    Text(Text, Charset),
    Video(Video)
}

impl MediaType {
    pub(super) fn build(&self) -> Vec<u8> {
        match self {
            Self::Application(application) => application.build().to_vec(),
            Self::Audio(audio)             =>       audio.build().to_vec(),
            Self::Image(image)             =>       image.build().to_vec(),
            Self::Video(video)             =>       video.build().to_vec(),
            Self::Text(text, charset)      => {
                let mut bytes = Vec::with_capacity(64);
                bytes.extend(text   .build());
                bytes.extend(charset.build());
                bytes
            }
        }
    }
}

pub enum Application {
    Binary,
    Gzip,
    Json,
    Php,
    Xhtml,
    Xml
}

impl Application {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Binary => b"Content-Type: application/octet-stream",
            Self::Gzip   => b"Content-Type: application/gzip",
            Self::Json   => b"Content-Type: application/json",
            Self::Php    => b"Content-Type: application/x-httpd-php",
            Self::Xhtml  => b"Content-Type: application/xhtml+xml",
            Self::Xml    => b"Content-Type: application/xml"
        }
    }
}

pub enum Audio {
    Aac,
    Midi,
    Mp3,
    Mp4,
    Ogg,
    Wav,
    Webm
}

impl Audio {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Aac  => b"Content-Type: audio/aac",
            Self::Midi => b"Content-Type: audio/midi",
            Self::Mp3  => b"Content-Type: audio/mpeg",
            Self::Mp4  => b"Content-Type: audio/mp4",
            Self::Ogg  => b"Content-Type: audio/ogg",
            Self::Wav  => b"Content-Type: audio/wav",
            Self::Webm => b"Content-Type: audio/webm"
        }
    }
}

pub enum Image {
    Apng,
    Avif,
    Bitmap,
    Gif,
    Icon,
    Jpeg,
    Png,
    Svg,
    Tiff,
    Webp
}

impl Image {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Apng   => b"Content-Type: image/apng",
            Self::Avif   => b"Content-Type: image/avif",
            Self::Bitmap => b"Content-Type: image/bmp",
            Self::Gif    => b"Content-Type: image/gif",
            Self::Icon   => b"Content-Type: image/x-icon",
            Self::Jpeg   => b"Content-Type: image/jpeg",
            Self::Png    => b"Content-Type: image/png",
            Self::Svg    => b"Content-Type: image/svg+xml",
            Self::Tiff   => b"Content-Type: image/tiff",
            Self::Webp   => b"Content-Type: image/webp"
        }
    }
}

pub enum Text {
    Css,
    Csv,
    Html,
    JavaScript,
    Markdown,
    Plain
}

impl Text {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Css        => b"Content-Type: text/css; ",
            Self::Csv        => b"Content-Type: text/csv; ",
            Self::Html       => b"Content-Type: text/html; ",
            Self::JavaScript => b"Content-Type: text/javascript; ",
            Self::Markdown   => b"Content-Type: text/markdown; ",
            Self::Plain      => b"Content-Type: text/plain; "
        }
    }
}

pub enum Video {
    Avi,
    Mp4,
    Mpeg,
    MpegStream,
    Ogg,
    Webm
}

impl Video {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Avi        => b"Content-Type: video/x-msvideo",
            Self::Mp4        => b"Content-Type: video/mp4",
            Self::Mpeg       => b"Content-Type: video/mpeg",
            Self::MpegStream => b"Content-Type: video/mp2t",
            Self::Ogg        => b"Content-Type: video/ogg",
            Self::Webm       => b"Content-Type: video/webm"
        }
    }
}

