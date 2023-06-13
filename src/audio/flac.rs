use std::{io::{Read, self}, error::Error, fs, ops::{Deref, DerefMut}};

pub enum BlockType {
    STREAMINFO,
    PADDING,
    APPLICATION,
    SEEKTABLE,
    VORBIS_COMMENT,
    CUESHEET,
    PICTURE
}

pub struct Flac {
    buffer: Vec<u8>
}

pub struct StreamInfo {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u32,
    max_frame_size: u32,
    sample_rate: u32,
    channels: u8,
    bits_per_sample: u8,
    samples: u64,
    md5: Vec<u8>
}

pub struct FlacMetadataBlock {
    header: FlacMetadataBlockHeader,
    data: FlacMetadataBlockData
}

pub struct FlacMetadataBlockHeader {
    last_block: bool,
    block_type: BlockType,
    len: u32
}

pub struct FlacMetadataBlockData {
    data: Vec<u8>
}

pub struct FlacStream(fs::File);
impl Deref for FlacStream {
    type Target = fs::File;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FlacStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<fs::File> for FlacStream {
    fn from(value: fs::File) -> Self {
        Self(value)
    }
}

impl Flac {
    pub fn load(mut stream: FlacStream) -> Result<Flac, Box<dyn Error>> {
        let mut magic_buf = [0u8; 4];
        stream.read(&mut magic_buf)?;
        if magic_buf != [0x66, 0x4C, 0x61, 0x43] {
            return Err("Provided file is not FLAC.".into());
        }
        println!("Is FLAC");

        Flac::parse(stream);
        todo!()
    }

    fn parse(mut stream: FlacStream) {

        todo!()
    }

    fn load_metadata(&mut self) {

    }
}