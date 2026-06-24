#[derive(Debug)]
pub enum Error {
    Generic(String),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
    IO(std::io::Error),
    FromUTF8(std::string::FromUtf8Error),
    Format(std::fmt::Error),
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error::Generic(string)
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Error::Regex(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::FromUTF8(error)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(fmt_error: std::fmt::Error) -> Self {
        Self::Format(fmt_error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Generic(s) => f.write_str(s),
            Error::Regex(e) => e.fmt(f),
            Error::ParseInt(e) => e.fmt(f),
            Error::IO(e) => e.fmt(f),
            Error::FromUTF8(e) => e.fmt(f),
            Error::Format(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Generic(_) => None,
            Error::Regex(e) => e.source(),
            Error::ParseInt(e) => e.source(),
            Error::IO(e) => e.source(),
            Error::FromUTF8(e) => e.source(),
            Error::Format(e) => e.source(),
        }
    }
}
