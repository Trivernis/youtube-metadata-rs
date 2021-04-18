use crate::error::YoutubeResult;
use crate::parsing::try_select_attribute;
use crate::types::VideoInformation;
use scraper::{Html, Selector};

lazy_static::lazy_static! {
    static ref TITLE_SELECTOR: Selector = Selector::parse(r#"meta[property="og:title"]"#).unwrap();
    static ref THUMBNAIL_SELECTOR: Selector = Selector::parse(r#"meta[property="og:image"]"#).unwrap();
    static ref URL_SELECTOR: Selector = Selector::parse(r#"link[rel="canonical"]"#).unwrap();
}

/// Parses information about a video from the html
pub fn parse_video_information(html: &str) -> YoutubeResult<VideoInformation> {
    let document = Html::parse_document(html);
    let url = try_select_attribute(&document, &URL_SELECTOR, "href")?;
    let title = try_select_attribute(&document, &TITLE_SELECTOR, "content")?;
    let thumbnail = try_select_attribute(&document, &THUMBNAIL_SELECTOR, "content").ok();

    Ok(VideoInformation {
        url: url.to_string(),
        title: title.to_string(),
        thumbnail: thumbnail.map(|s| s.to_string()),
    })
}
