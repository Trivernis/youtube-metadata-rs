use crate::get_video_information;

#[tokio::test]
async fn invalid_url_is_err() {
    assert!(
        get_video_information("https://www.youtube.com/watch?v=FFFFFFFFFFF")
            .await
            .is_err()
    );
}
