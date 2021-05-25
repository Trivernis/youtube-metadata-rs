use crate::error::Result;
use crate::parsing::video_information::parse_video_information;
use crate::types::VideoInformation;

/// Returns information about a video
/// ```
/// use youtube_metadata::get_video_information;
/// # #[tokio::test]
/// # async fn doctest() {
/// let information = get_video_information("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
///      .await
///      .unwrap();
///  assert_eq!(information.id, "dQw4w9WgXcQ".to_string());
///  assert_eq!(
///      information.url,
///      "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
///  );
///  assert_eq!(information.uploader, "RickAstleyVEVO".to_string());
///  assert_eq!(
///      information.title,
///      "Rick Astley - Never Gonna Give You Up (Video)".to_string()
///  );
///  assert_eq!(
///      information.thumbnail,
///      Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string())
///  );
/// # }
/// ```
pub async fn get_video_information(url: &str) -> Result<VideoInformation> {
    let response = reqwest::get(url).await?;
    let response_text = response.text().await?;

    parse_video_information(&response_text)
}
