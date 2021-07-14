//! Search related models.

use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{
    id::{PlaylistId, VideoId},
    Channel, Video,
};

/// Search result contents.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SearchResult {
    /// List of search result items.
    pub items: Vec<SearchItem>,
}

impl SearchResult {
    /// Filters the results to an [`Iterator`] over [`Video`]s.
    pub fn videos(self) -> impl Iterator<Item = Video> {
        self.items.into_iter().filter_map(|item| match item {
            SearchItem::Playlist(_) => None,
            SearchItem::Video(v) => Some(v),
        })
    }

    /// Filters the results to an [`Iterator`] over [`PartialPlaylist`]s.
    pub fn playlists(self) -> impl Iterator<Item = PartialPlaylist> {
        self.items.into_iter().filter_map(|item| match item {
            SearchItem::Playlist(p) => Some(p),
            SearchItem::Video(_) => None,
        })
    }
}

/// Contains the possible item for a search.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SearchItem {
    /// Playlist item.
    ///
    /// Note that playlists don't contain a [`Vec<Video>`] but rather a [`Vec<PartialPlaylistVideo>`]
    /// (which is missing a [`Channel`] field).
    ///
    /// This [`Vec`] also only contains the first two items.
    /// Thus to get the full list of videos in a playlist another query has to be used using the
    /// [`PlaylistId`] (note that this is not implemented yet for this library).
    ///
    /// [`PlaylistId`]: super::PlaylistId
    Playlist(PartialPlaylist),
    /// Video item.
    Video(Video),
}

impl SearchItem {
    /// Returns an immutable reference to the name of the inner item.
    pub fn title(&self) -> &str {
        match self {
            SearchItem::Playlist(p) => &p.title,
            SearchItem::Video(v) => &v.title,
        }
    }

    /// Returns an immutable reference to the uploader of the innner item.
    pub fn uploader(&self) -> &Channel {
        match self {
            SearchItem::Playlist(p) => &p.uploader,
            SearchItem::Video(v) => &v.uploader,
        }
    }
}

/// Information about a partial playlist.
///
/// This struct is returned from searches.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PartialPlaylist {
    /// The playlist's unique Id.
    pub id: PlaylistId,
    /// Up to the first two tracks of the playlist.
    pub tracks: Vec<PartialPlaylistVideo>,
    /// The total number of tracks in the playlist.
    pub tracks_total: u32,
    /// The playlist's title.
    pub title: String,
    /// The playlist's uploader.
    pub uploader: Channel,
}

/// Information about a video in a [`PartialPlaylist`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PartialPlaylistVideo {
    /// The video's unique Id.
    pub id: VideoId,
    /// The video's length.
    pub length: Duration,
    /// The video's title.
    pub title: String,
}
