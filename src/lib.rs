pub mod flac;

#[derive(Debug, Clone)]
pub struct BasicMetadata {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
}

pub trait Metadata {
    fn title(&self) -> Option<String>;
    fn artist(&self) -> Option<String>;
    fn album(&self) -> Option<String>;
    fn metadata(&self) -> Option<BasicMetadata>;
}

pub trait Cover {
    type CoverType;
    fn cover(&self) -> Self::CoverType;
}