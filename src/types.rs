#[derive(Clone, Debug)]
pub struct VideoInformation {
    pub id: String,
    pub url: String,
    pub title: String,
    pub uploader: String,
    pub thumbnail: Option<String>,
}
