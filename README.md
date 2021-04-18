# youtube-metadata-rs

Fetch simple YouTube Video metadata with ease.


## Example

```rust
let information = get_video_information("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
    .await
    .unwrap();
assert_eq!(information.id, "dQw4w9WgXcQ".to_string());
assert_eq!(
    information.url,
    "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
);
assert_eq!(information.uploader, "RickAstleyVEVO".to_string());
assert_eq!(
    information.title,
    "Rick Astley - Never Gonna Give You Up (Video)".to_string()
);
assert_eq!(
    information.thumbnail,
    Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg".to_string())
);
```


## License

MIT
