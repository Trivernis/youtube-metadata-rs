use crate::endpoints::get_video_information;

#[tokio::test]
async fn test_get_video_information() {
    let information = get_video_information("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
        .await
        .unwrap();
    assert_eq!(
        information.url,
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
    );
    assert_eq!(
        information.title,
        "Rick Astley - Never Gonna Give You Up (Video)".to_string()
    );
    assert_eq!(
        information.thumbnail,
        Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string())
    );

    assert!(
        get_video_information("https://www.youtube.com/watch?v=FFFFFFFFFFF")
            .await
            .is_err()
    );
}
