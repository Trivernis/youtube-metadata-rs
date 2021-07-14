//! Thumbnail configuration types.
//!
//! Invalid thumbnails resolve to this image:
//!
//! ![](https://i.ytimg.com)
//!
//! See [YouTube's API docs][docs] for more info.
//!
//! [docs]: https://developers.google.com/youtube/v3/docs/thumbnails
//! [`VideoId`]: super::id::VideoId

use std::fmt;

/// Base url of thumbnails.
pub(crate) const URL: &str = "https://i.ytimg.com";

/// YouTube's supported image formats.
///
/// [`ImageFormat::WebP`] retains the same *or better* quality at a smaller size.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ImageFormat {
    /// Older, better supported format.
    JPEG,
    /// Newer (2010) and more efficient format.
    WebP,
}

/// YouTube's resolution types.
///
/// Query the YouTube API to know if [`Resolution::Maxres`] or [`Resolution::Standard`] are
/// available for a resource.
///
/// The resolution varies on which resource the thumbnail is for, video resolution's are currently
/// documented.
#[derive(Clone, Copy, Debug)]
pub enum Resolution {
    /// 120 x 90px
    Default,
    /// 480 x 360px
    High,
    /// 1280 x 720px
    ///
    /// Not available for all resources.
    Maxres,
    /// 320 x 180px
    Medium,
    /// 640 x 480px
    ///
    /// Not available for all resources.
    Standard,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resolution::Default => (),
            Resolution::High => f.write_str("hq")?,
            Resolution::Maxres => f.write_str("maxres")?,
            Resolution::Medium => f.write_str("mq")?,
            Resolution::Standard => f.write_str("sd")?,
        }
        f.write_str("default")
    }
}
