use std::time::Duration;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::{
    error::{Error, ParseError, ParseErrorKind},
    model::{
        id::{ChannelId, VideoId},
        Channel, Video,
    },
};

#[macro_use]
macro_rules! other {
    () => {
        ParseError {
            kind: ParseErrorKind::Other,
        }
    };
}

pub(crate) mod search;

fn yt_initial_data(html: &str) -> Result<Value, Error> {
    lazy_static! {
        // FIXME: improve against accidental termination
        static ref RE: Regex = Regex::new(r"var ytInitialData = (.*?);</script>").expect("valid regex");
    }

    serde_json::from_str::<Value>(
        RE.captures(html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .ok_or(ParseError {
                kind: ParseErrorKind::Regex,
            })?,
    )
    .map_err(|_| {
        ParseError {
            kind: ParseErrorKind::Regex,
        }
        .into()
    })
}

fn yt_initial_player_response(html: &str) -> Result<Value, Error> {
    lazy_static! {
        // FIXME: improve against accidental termination
        static ref RE: Regex = Regex::new(r"var ytInitialPlayerResponse = (.*);</script>").expect("valid regex");
    }

    serde_json::from_str::<Value>(
        RE.captures(html)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str())
            .ok_or(ParseError {
                kind: ParseErrorKind::Regex,
            })?,
    )
    .map_err(|_| {
        ParseError {
            kind: ParseErrorKind::Regex,
        }
        .into()
    })
}

pub(crate) fn video_information(html: &str) -> Result<Video, Error> {
    let data = yt_initial_data(html)?;
    let player = yt_initial_player_response(html)?;

    let items = data
        .pointer("/contents/twoColumnWatchNextResults/results/results/contents")
        .ok_or(other!())?;

    let id = VideoId::new(
        data.pointer("/currentVideoEndpoint/watchEndpoint/videoId")
            .and_then(Value::as_str)
            .ok_or(other!())?,
    );

    let length = player
        .pointer("/streamingData/formats/0/approxDurationMs")
        .and_then(Value::as_str)
        .and_then(|ms| ms.parse::<u64>().ok())
        .map(Duration::from_millis)
        .ok_or(other!())?;

    let title = items
        .pointer("/0/videoPrimaryInfoRenderer/title/runs/0/text")
        .and_then(Value::as_str)
        .ok_or(other!())?
        .to_owned();

    let uploader = items
        .pointer("/1/videoSecondaryInfoRenderer/owner/videoOwnerRenderer/title/runs/0")
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

    Ok(Video {
        id,
        length,
        title,
        uploader,
    })
}

fn video_is_live(video: &Value) -> bool {
    let badges = video.get("badges").and_then(Value::as_array);
    badges
        .map(|badges| {
            badges.iter().any(|badge| {
                badge
                    .pointer("/metadataBadgeRenderer/style")
                    .and_then(Value::as_str)
                    .eq(&Some("BADGE_STYLE_TYPE_LIVE_NOW"))
            })
        })
        .unwrap_or_default()
}

fn length_to_dur(input: &str) -> Duration {
    fn time_multiplier(i: usize) -> u64 {
        match i {
            0 => 1,
            1 => 60,
            2 => 3600,
            _ => unreachable!("YouTube duration's aren't counted in days"),
        }
    }
    let mut duration = Duration::ZERO;
    for (i, time) in input
        .split(':')
        .map(|s| s.parse::<u64>().expect("is str encoded integer"))
        .rev()
        .enumerate()
    {
        duration += Duration::from_secs(time * time_multiplier(i))
    }
    duration
}
