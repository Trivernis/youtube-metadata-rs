//! Mapping of output.
//!
//! Use the resource's id's to get thumbnails or urls.

use std::time::Duration;

use id::{PlaylistId, VideoId};
use search::{PartialPlaylist, PartialPlaylistVideo};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::id::ChannelId;

pub mod id;
pub mod search;
pub mod thumbnail;

/// Information about a channel.
///
/// Note that this is *not* a user so its [`Channel::id`] is of the form of `/channel/ID`, not
/// `/user/ID`.
/// The link still resolves to the same page, so this should not be an issue in most cases.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Channel {
    /// The channel's unique Id.
    pub id: ChannelId,
    /// The channel's name.
    pub name: String,
}

/// Information about a playlist.
// Hide since not implemented.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Playlist {
    /// The playlist's unique Id.
    pub id: PlaylistId,
    /// The playlist's tracks.
    pub tracks: Vec<Video>,
    /// The playlist's title.
    pub title: String,
    /// The playlists's uploader.
    pub uploader: Channel,
}

impl From<Playlist> for PartialPlaylist {
    fn from(mut p: Playlist) -> Self {
        // partial playlist only contains up to the first two videos
        let tracks_total = p.tracks.len() as u32;
        p.tracks.truncate(2);
        let tracks = p.tracks.into_iter().map(Into::into).collect();
        Self {
            id: p.id,
            tracks,
            tracks_total,
            title: p.title,
            uploader: p.uploader,
        }
    }
}

/// Resource types.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Resource {
    /// Playlist type.
    ///
    /// Note that this variant is never constructed (not implemented).
    Playlist(Playlist),
    /// Video type.
    Video(Video),
}

/// Information about a video.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Video {
    /// The video's unique Id.
    pub id: VideoId,
    /// The video's length.
    pub length: Duration,
    /// The video's title.
    pub title: String,
    /// The video's uploader.
    pub uploader: Channel,
}

impl From<Video> for PartialPlaylistVideo {
    fn from(v: Video) -> Self {
        Self {
            id: v.id,
            length: v.length,
            title: v.title,
        }
    }
}
