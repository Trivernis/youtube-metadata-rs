use crate::{
    error::Error,
    model::{id::VideoId, search::SearchResult, Resource, Video},
    parsing::{search::search_information, video_information},
};

/// Reusable client, [`NotReusable`]s cousin.
///
/// Internally wraps around an [`Arc`], so cloning is cheap.
///
/// [`Arc`]: std::sync::Arc
#[derive(Clone, Debug, Default)]
pub struct Reusable(reqwest::Client);

impl Reusable {
    /// Create a new reusable client.
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }

    // Not implemented
    /*
     * /// Get a playlist by its id.
     * pub async fn playlist(&self, playlist: PlaylistId) -> Result<Playlist, Error> {
     *     todo!()
     * }
     */

    /// Search for some query on youtube
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use youtube_metadata::Reusable;
    /// use std::time::Duration;
    ///
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let reusable = Reusable::new();
    /// let first = reusable.search("Rick Astley - Never Gonna Give You Up (Official Music Video)")
    ///     .await?
    ///     .videos()
    ///     .next()
    ///     .expect("atleast one result");
    /// assert_eq!(first.id.as_str(), "dQw4w9WgXcQ");
    /// assert_eq!(first.length, Duration::from_secs(213));
    /// assert_eq!(first.title,
    /// String::from("Rick Astley - Never Gonna Give You Up (Official Music Video)"));
    /// assert_eq!(first.uploader.name, "Rick Astley");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, search: &str) -> Result<SearchResult, Error> {
        let request = self
            .0
            .get("https://youtube.com/results?")
            .query(&[("q", search)])
            .build()?;

        let response_text = self.0.execute(request).await?.text().await?;

        search_information(&response_text)
    }

    /// Get a video by its id.
    pub async fn video(&self, video: VideoId) -> Result<Video, Error> {
        let url = format!("https://www.youtube.com/watch?v={}", video);
        match self.query(&url).await? {
            Resource::Video(v) => (Ok(v)),
            _ => unreachable!(),
        }
    }

    /// Fetch a resource from a url.
    ///
    /// Will only resolve to [`Resource::Video`] right now due to playlists being unsupported.
    ///
    /// [`Resource`] will currently only contain a video due to playlists being unimplemented.
    pub async fn query(&self, query: &str) -> Result<Resource, Error> {
        let request = self.0.get(query).build()?;

        let response_text = self.0.execute(request).await?.text().await?;

        // for now call this since only videos are supported.
        Ok(Resource::Video(video_information(&response_text)?))
    }
}

/// Zero sized associated function holder, [`Reusable`]s cousin.
///
/// Creates a new client on each invocation.
#[derive(Debug)]
pub struct NotReusable;

impl NotReusable {
    // Not implemented
    /*
     * /// Get a playlist by its id.
     * pub async fn playlist(playlist: PlaylistId) -> Result<Playlist, Error> {
     *     todo!()
     * }
     */

    /// Search for some query on youtube
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use youtube_metadata::NotReusable;
    /// #
    /// use std::time::Duration;
    ///
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let first = NotReusable::search("Rick Astley - Never Gonna Give You Up (Official Music Video)")
    ///     .await?
    ///     .videos()
    ///     .next()
    ///     .expect("atleast one result");
    /// assert_eq!(first.id.as_str(), "dQw4w9WgXcQ");
    /// assert_eq!(first.length, Duration::from_secs(213));
    /// assert_eq!(first.title,
    /// String::from("Rick Astley - Never Gonna Give You Up (Official Music Video)"));
    /// assert_eq!(first.uploader.name, "Rick Astley");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(search: &str) -> Result<SearchResult, Error> {
        let client = reqwest::Client::new();

        let request = client
            .get("https://youtube.com/results?")
            .query(&[("q", search)])
            .build()?;

        let response_text = client.execute(request).await?.text().await?;

        search_information(&response_text)
    }

    /// Get a video by its id.
    pub async fn video(video: VideoId) -> Result<Video, Error> {
        let url = format!("https://www.youtube.com/watch?v={}", video);
        match Self::query(&url).await? {
            Resource::Video(v) => (Ok(v)),
            _ => unreachable!(),
        }
    }

    /// Fetch a resource from a url.
    ///
    /// Will only resolve to [`Resource::Video`] right now due to playlists being unsupported.
    ///
    /// [`Resource`] will currently only contain a video due to playlists being unimplemented.
    pub async fn query(query: &str) -> Result<Resource, Error> {
        let client = reqwest::Client::new();

        let request = client.get(query).build()?;

        let response_text = client.execute(request).await?.text().await?;

        // for now call this since only videos are supported.
        Ok(Resource::Video(video_information(&response_text)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    #[tokio::test]
    async fn rickroll() -> Result<(), Box<dyn std::error::Error>> {
        let search =
            NotReusable::search("Rick Astley - Never Gonna Give You Up (Official Music Video)")
                .await?
                .videos()
                .next()
                .expect("atleast one result");
        let video = NotReusable::video(VideoId::new("dQw4w9WgXcQ")).await?;

        assert_eq!(search.id.as_str(), "dQw4w9WgXcQ");
        assert_eq!(video.id.as_str(), "dQw4w9WgXcQ");
        assert_eq!(search.length, Duration::from_secs(213));
        assert_eq!(video.length, Duration::from_millis(212091));
        assert_eq!(
            search.title.as_str(),
            "Rick Astley - Never Gonna Give You Up (Official Music Video)"
        );
        assert_eq!(
            video.title.as_str(),
            "Rick Astley - Never Gonna Give You Up (Official Music Video)"
        );
        assert_eq!(search.uploader.name, "Rick Astley");
        assert_eq!(video.uploader.name, "Rick Astley");
        Ok(())
    }

    #[tokio::test]
    async fn live() -> Result<(), Box<dyn std::error::Error>> {
        NotReusable::search("live music").await?;

        Ok(())
    }

    #[tokio::test]
    async fn playlist() -> Result<(), Box<dyn std::error::Error>> {
        NotReusable::search("music playlist").await?;

        Ok(())
    }
}
