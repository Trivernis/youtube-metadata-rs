use lazy_static::lazy_static;
use scraper::{Html, Selector};

use super::try_select_attribute;
use crate::{error::Result, types::VideoInformation};

lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();
    static ref THUMBNAIL_SELECTOR: Selector = Selector::parse(r#"meta[property="og:image"]"#).unwrap();
    static ref URL_SELECTOR: Selector = Selector::parse(r#"link[rel="canonical"]"#).unwrap();
    static ref CHANNEL_SELECTOR: Selector = Selector::parse(r#"link[itemprop="name"]"#).unwrap();
    static ref ID_SELECTOR: Selector = Selector::parse(r#"meta[itemprop="videoId"]"#).unwrap();
}

/// Parses information about a video from the html
pub fn parse_video_information(html: &str) -> Result<VideoInformation> {
    let document = Html::parse_document(html);

    let video_id = try_select_attribute(&document, &ID_SELECTOR, "content")?;
    let url = try_select_attribute(&document, &URL_SELECTOR, "href")?;
    let author = try_select_attribute(&document, &CHANNEL_SELECTOR, "content")?;
    let title = try_select_attribute(&document, &TITLE_SELECTOR, "content")?;
    let thumbnail = try_select_attribute(&document, &THUMBNAIL_SELECTOR, "content").ok();

    Ok(VideoInformation {
        id: video_id.to_string(),
        url: url.to_string(),
        title: title.to_string(),
        uploader: author.to_string(),
        thumbnail: thumbnail.map(|s| s.to_string()),
    })
}
