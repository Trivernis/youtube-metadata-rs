use thiserror::Error;

pub type YoutubeResult<T> = Result<T, YoutubeError>;

#[derive(Debug, Error)]
pub enum YoutubeError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Parse Error: {0}")]
    ParseError(String),
}

impl From<&str> for YoutubeError {
    fn from(s: &str) -> Self {
        Self::ParseError(s.to_string())
    }
}
