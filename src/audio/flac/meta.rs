use std::collections::HashMap;

#[derive(Debug)]
pub struct StreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub samples: u64,
    pub md5: u128
}

#[derive(Debug)]
pub struct VorbisComment {
    pub vendor_string: String,
    pub comments: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub enum PictureType {
    Other,
    FileIcon,
    OtherFileIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    Lead,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    Location,
    Recording,
    Performance,
    MovieCapture,
    Fish,
    Illustration,
    ArtistLogo,
    StudioLogo
}

#[derive(Debug)]
pub struct Picture {
    pub picture_type: PictureType,
    pub mime_type: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub colour_count: u32,
    pub buffer: Vec<u8>
}