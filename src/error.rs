//! Error types of this library.
//!
//! Note that parsing should never fail and is indicative of an interal error.
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

use reqwest::Error as ReqwestError;

#[derive(Debug)]
#[doc(hidden)]
pub struct ParseError {
    pub(crate) kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ParseErrorKind::*;

        match self.kind {
            Other => f.write_str("something failed to parse"),
            Regex => f.write_str("fetching json using regex failed"),
        }
    }
}

impl StdError for ParseError {}

#[derive(Debug)]
pub(crate) enum ParseErrorKind {
    Other,
    Regex,
}

/// Error types.
#[derive(Debug)]
pub enum Error {
    /// Error doing http.
    Reqwest(ReqwestError),
    /// Internal parsing error.
    /// Hitting this should never happen and is a bug.
    Parse(ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Error::*;
        match self {
            Reqwest(e) => e.fmt(f),
            Parse(_) => write!(f, "json parsing error"),
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

impl From<ReqwestError> for Error {
    fn from(e: ReqwestError) -> Self {
        Self::Reqwest(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}
