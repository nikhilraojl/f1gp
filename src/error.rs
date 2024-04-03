use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    IO(std::io::Error),
    SerdeJson(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fmt(err) => write!(fmt, "{err}"),
            Self::IO(err) => write!(fmt, "{err}"),
            Self::SerdeJson(err) => write!(fmt, "{err}"),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
