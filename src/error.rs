#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Str(String),
    Str2(std::string::FromUtf8Error),
}

impl Into<std::io::Error> for Error {
    fn into(self) -> std::io::Error {
        match self {
            Error::Io(err) => err,
            Error::Str(s) => std::io::Error::new(std::io::ErrorKind::Other, s),
            Error::Str2(s) => std::io::Error::new(std::io::ErrorKind::Other, s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error::Str2(e)
    }
}
