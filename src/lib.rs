pub mod flac;

#[derive(Debug)]
pub struct BasicMetadata {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
}