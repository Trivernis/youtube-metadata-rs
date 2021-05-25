use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

use reqwest::Error as ReqwestError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(ReqwestError),
    Parse(Parsing),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;
        match self {
            Reqwest(e) => e.fmt(f),
            Parse(_) => write!(f, "parse error"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use Error::*;
        match self {
            Reqwest(e) => e.source(),
            Parse(e) => Some(e),
        }
    }
}

impl From<Parsing> for Error {
    fn from(s: Parsing) -> Self {
        Self::Parse(s)
    }
}

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

#[derive(Debug)]
pub enum Parsing {
    MissingElement(String),
    MissingAttribute(String),
}

impl Display for Parsing {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Parsing::*;
        match self {
            MissingAttribute(s) => write!(f, "missing attribute: {}", s),
            MissingElement(s) => write!(f, "missing element: {}", s),
        }
    }
}

impl StdError for Parsing {}
