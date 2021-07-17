//! Type-safe resource identifiers.
//!
//! Note that a `thumbnail` method is unavailable for [`PlaylistId`], this is due to playlist's
//! using their first video's thumbnail.
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::thumbnail;

/// Channel identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChannelId(String);

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl ChannelId {
    pub(crate) fn new(id: &str) -> Self {
        Self(id.to_owned())
    }

    /// Yields the underyling string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the id, returning the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get the channel url
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/channel/{}", self)
    }
}

/// Playlist identifier.
///
/// Use [`PlaylistId::url`] to get the playlist url.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaylistId(String);

impl fmt::Display for PlaylistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PlaylistId {
    pub(crate) fn new(id: &str) -> Self {
        Self(id.to_owned())
    }

    /// Yields the underyling string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the id, returning the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get the playlist url
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/playlist?list={}", self)
    }
}

/// Video identifier.
// TODO: feature flag for staticvec (allows `Copy`) (requires nightly)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VideoId(String);

impl fmt::Display for VideoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl VideoId {
    pub(crate) fn new(id: &str) -> Self {
        Self(id.to_owned())
    }

    /// Yields the underyling string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the id, returning the underlying string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Get the thumbnail url.
    pub fn thumbnail(&self, format: thumbnail::ImageFormat, res: thumbnail::Resolution) -> String {
        match format {
            thumbnail::ImageFormat::JPEG => {
                format!("{}/vi/{}/{}.jpg", thumbnail::URL, self, res)
            }
            thumbnail::ImageFormat::WebP => {
                format!("{}/vi_webp/{}/{}.webp", thumbnail::URL, self, res)
            }
        }
    }

    /// Get the video url.
    pub fn url(&self) -> String {
        format!("https://youtu.be/{}", self)
    }
}
