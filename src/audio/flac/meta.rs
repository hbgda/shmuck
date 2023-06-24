//! Reference: https://xiph.org/flac/format.html

use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Clone)]
/// https://xiph.org/flac/format.html#metadata_block_streaminfo
pub struct StreamInfo {
    pub min_block_size: u16,
    pub max_block_size: u16,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u8,
    pub samples: u64,
    pub md5: Vec<u8>
}

#[derive(Debug, Clone, Copy)]
/// https://xiph.org/flac/format.html#metadata_block_padding
pub struct Padding(usize);
impl Deref for Padding {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
/// https://xiph.org/flac/format.html#metadata_block_vorbis_comment
pub struct VorbisComment {
    pub vendor_string: String,
    pub comments: HashMap<String, Vec<String>>
}

#[derive(Debug, Clone)]
/// https://xiph.org/flac/format.html#metadata_block_picture
pub struct Picture {
    pub picture_type: PictureType,
    pub mime_type: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub colour_depth: u32,
    pub colour_count: u32,
    pub buffer: Vec<u8>
}

#[derive(Debug, Clone, Copy)]
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
    StudioLogo,
    Reserved
}

impl From<u32> for PictureType {
    fn from(value: u32) -> Self {
        match value {
            0 => PictureType::Other,
            1 => PictureType::FileIcon,
            2 => PictureType::OtherFileIcon,
            3 => PictureType::CoverFront,
            4 => PictureType::CoverBack,
            5 => PictureType::Leaflet,
            6 => PictureType::Media,
            7 => PictureType::Lead,
            8 => PictureType::Artist,
            9 => PictureType::Conductor,
            10 => PictureType::Band,
            11 => PictureType::Composer,
            12 => PictureType::Lyricist,
            13 => PictureType::Location,
            14 => PictureType::Recording,
            15 => PictureType::Performance,
            16 => PictureType::MovieCapture,
            17 => PictureType::Fish,
            18 => PictureType::Illustration,
            19 => PictureType::ArtistLogo,
            20 => PictureType::StudioLogo,
            _ => PictureType::Reserved
        }
    }
}