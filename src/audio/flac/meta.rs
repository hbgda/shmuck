use std::collections::HashMap;

#[derive(Debug)]
pub struct StreamInfo {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_rate: u32,
    channels: u8,
    bits_per_sample: u8,
    samples: u64,
    md5: u128
}

#[derive(Debug)]
pub struct VorbisComment {
    vendor_string: String,
    comments: HashMap<String, Vec<String>>
}

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
    picture_type
}