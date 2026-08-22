// mochou-p/rspond/src/mime_type.rs

pub enum MimeType {
    Application(Application),
    Audio(Audio),
    Image(Image),
    Text(Text)
}

impl MimeType {
    pub(super) fn build(&self) -> &'static [u8] {
        match self {
            Self::Application(application) => application.build(),
            Self::Audio(audio)             =>       audio.build(),
            Self::Image(image)             =>       image.build(),
            Self::Text(text)               =>        text.build()
        }
    }
}

pub enum Application {
    Xhtml
}

impl Application {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Xhtml => b"Content-Type: application/xhtml+xml; "
        }
    }
}

pub enum Audio {
    Mpeg
}

impl Audio {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Mpeg => b"Content-Type: audio/mpeg; "
        }
    }
}

pub enum Image {
    Jpeg
}

impl Image {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Jpeg => b"Content-Type: image/jpeg; "
        }
    }
}

pub enum Text {
    Html,
    Css
}

impl Text {
    fn build(&self) -> &'static [u8] {
        match self {
            Self::Html => b"Content-Type: text/html; ",
            Self::Css  => b"Content-Type: text/css; "
        }
    }
}

