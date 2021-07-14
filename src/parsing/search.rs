use serde_json::Value;
use tracing::{event, instrument, Level};

use crate::{
    error::{Error, ParseError, ParseErrorKind},
    model::{
        id::{ChannelId, PlaylistId, VideoId},
        search::{PartialPlaylist, PartialPlaylistVideo, SearchItem, SearchResult},
        Channel, Video,
    },
};

use super::{length_to_dur, video_is_live, yt_initial_data};

pub(crate) fn search_information(html: &str) -> Result<SearchResult, Error> {
    let data = yt_initial_data(html)?;
    let items = data
        .pointer("/contents/twoColumnSearchResultsRenderer/primaryContents/sectionListRenderer/contents/0/itemSectionRenderer/contents")
        .and_then(Value::as_array)
        .ok_or(other!())?;

    let mut things = Vec::with_capacity(items.len());
    for item in items {
        if let Some(video) = item.get("videoRenderer") {
            let video = match parse_video(video)? {
                Some(v) => SearchItem::Video(v),
                None => continue,
            };
            things.push(video);
        } else if let Some(playlist) = item.get("playlistRenderer") {
            let playlist = match parse_playlist(playlist)? {
                Some(p) => SearchItem::Playlist(p),
                None => continue,
            };
            things.push(playlist);
        } else if let Some(_shelf) = item.get("shelfRenderer") {
            //println!("{:?}", shelf.pointer("/title/simpleText"));
            // TODO: "shelfRender" & "radioRender"
        } else {
            // TODO: "radioRender?"
        }
    }

    Ok(SearchResult { items: things })
}

#[instrument(skip(video), fields(id))]
fn parse_video(video: &Value) -> Result<Option<Video>, Error> {
    let id = VideoId::new(
        video
            .get("videoId")
            .and_then(Value::as_str)
            .ok_or(other!())?,
    );

    tracing::Span::current().record("id", &id.as_str());

    // skip live (for now)
    if video_is_live(video) {
        event!(Level::TRACE, "skipping live stream");
        return Ok(None);
    }

    let length = match video
        .pointer("/lengthText/simpleText")
        .and_then(Value::as_str)
    {
        Some(l) => length_to_dur(l),
        // Live badge is sometimes missing for no reason
        None => {
            event!(
                Level::DEBUG,
                "video without length (livestream?) found, skipping..."
            );
            return Ok(None);
        }
    };

    let title = video
        .pointer("/title/runs/0/text")
        .and_then(Value::as_str)
        .ok_or(other!())?
        .to_owned();

    let uploader = video.pointer("/ownerText/runs/0").ok_or(other!())?;
    let uploader = Channel {
        id: ChannelId::new(
            uploader
                .pointer("/navigationEndpoint/browseEndpoint/browseId")
                .and_then(Value::as_str)
                .ok_or(other!())?,
        ),
        name: uploader
            .get("text")
            .and_then(Value::as_str)
            .ok_or(other!())?
            .to_owned(),
    };

    Ok(Some(Video {
        id,
        length,
        title,
        uploader,
    }))
}

#[instrument(skip(playlist), fields(id))]
fn parse_playlist(playlist: &Value) -> Result<Option<PartialPlaylist>, Error> {
    let id = PlaylistId::new(
        playlist
            .get("playlistId")
            .and_then(Value::as_str)
            .ok_or(other!())?,
    );

    tracing::Span::current().record("id", &id.as_str());

    let items = playlist
        .get("videos")
        .and_then(Value::as_array)
        .ok_or(other!())?;
    let mut tracks = Vec::with_capacity(items.len());
    for track in items {
        let video = track.get("childVideoRenderer").ok_or(other!())?;

        let id = VideoId::new(
            video
                .get("videoId")
                .and_then(Value::as_str)
                .ok_or(other!())?,
        );

        let length = length_to_dur(
            video
                .pointer("/lengthText/simpleText")
                .and_then(Value::as_str)
                .ok_or(other!())?,
        );

        let title = video
            .pointer("/title/simpleText")
            .and_then(Value::as_str)
            .unwrap()
            .to_owned();

        let obj = PartialPlaylistVideo { id, length, title };
        tracks.push(obj);
    }

    let tracks_total = playlist
        .pointer("/videoCountText/runs/0/text")
        .and_then(Value::as_str)
        .ok_or(other!())?
        .parse::<u32>()
        .map_err(|_| other!())?;

    let title = playlist
        .pointer("/title/simpleText")
        .and_then(Value::as_str)
        .ok_or(other!())?
        .to_owned();

    let uploader = playlist
        .pointer("/shortBylineText/runs/0")
        .ok_or(other!())?;
    let uploader = Channel {
        id: ChannelId::new(
            uploader
                .pointer("/navigationEndpoint/browseEndpoint/browseId")
                .and_then(Value::as_str)
                .ok_or(other!())?,
        ),
        name: uploader
            .get("text")
            .and_then(Value::as_str)
            .ok_or(other!())?
            .to_owned(),
    };

    Ok(Some(PartialPlaylist {
        id,
        tracks,
        tracks_total,
        title,
        uploader,
    }))
}
