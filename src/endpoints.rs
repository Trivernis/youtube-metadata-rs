use crate::error::YoutubeResult;
use crate::parsing::video_information::parse_video_information;
use crate::types::VideoInformation;

/// Returns information about a video
pub async fn get_video_information(url: &str) -> YoutubeResult<VideoInformation> {
    let response = reqwest::get(url).await?;
    let response_text = response.text().await?;

    parse_video_information(&response_text)
}
