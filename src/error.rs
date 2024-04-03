use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Fmt(std::fmt::Error),
    IO(std::io::Error),
    SerdeJson(serde_json::Error),
    Ureq(Box<ureq::Error>),
    ParseInt(std::num::ParseIntError),
    Scraper,
    ParserDriverInfo,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fmt(err) => write!(fmt, "{err}"),
            Self::IO(err) => write!(fmt, "{err}"),
            Self::SerdeJson(err) => write!(fmt, "{err}"),
            Self::Ureq(err) => write!(fmt, "{err}"),
            Self::Scraper => write!(fmt, "HTML parsing failed"),
            Self::ParseInt(err) => write!(fmt, "{err}"),
            Self::ParserDriverInfo => write!(fmt, "Driver table row parsing failed"),
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
impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Self::Ureq(Box::new(err))
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
