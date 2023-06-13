use std::ops::Deref;

#[derive(Debug)]
pub enum BlockType {
    STREAMINFO,
    PADDING,
    APPLICATION,
    SEEKTABLE,
    VORBIS_COMMENT,
    CUESHEET,
    PICTURE,
    RESERVED
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

pub enum InnerBlockData {
    STREAMINFO(StreamInfo)
}

pub struct MetadataBlock {
    pub header: BlockHeader,
    pub data: BlockData
}

#[derive(Debug)]
pub struct BlockHeader {
    pub last_block: bool,
    pub block_type: BlockType,
    pub len: u32
}

pub struct BlockData(InnerBlockData);
impl Deref for BlockData {
    type Target = InnerBlockData;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BlockHeader {
    pub fn parse(mut buf: Vec<u8>) -> BlockHeader {
        let last_block = buf[0] >> 7 == 1;
        let block_type = ((buf[0] << 1) >> 1).into();
        buf[0] = 0;
        let len = u32::from_be_bytes(buf.try_into().unwrap());
        
        dbg!(BlockHeader { last_block, block_type, len })
    }
}

impl BlockData {
    pub fn parse(buf: Vec<u8>, block_type: BlockType) -> BlockData {
        let block = match block_type {
            BlockType::STREAMINFO => {
                BlockData::_parse_stream_info(buf)
            }
            _ => todo!()
        };

        BlockData(block)
    }

    fn _parse_stream_info(buf: Vec<u8>) -> InnerBlockData {
        let min_block_size = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        let max_block_size = u16::from_be_bytes(buf[2..4].try_into().unwrap());

        let mut min_frame_buf = [0u8; 4];
        min_frame_buf[1..].copy_from_slice(&buf[4..7]);
        let minimum_frame_size = u32::from_be_bytes(min_frame_buf);
        
        let mut max_frame_buf = [0u8; 4];
        max_frame_buf[1..].copy_from_slice(&buf[7..10]);
        let maximum_frame_size = u32::from_be_bytes(max_frame_buf);

        dbg!(min_block_size, max_block_size, minimum_frame_size, maximum_frame_size);
        todo!()
    }
}

impl From<u8> for BlockType {
    fn from(value: u8) -> Self {
        match value {
            0 => BlockType::STREAMINFO,
            1 => BlockType::PADDING,
            2 => BlockType::APPLICATION,
            3 => BlockType::SEEKTABLE,
            4 => BlockType::VORBIS_COMMENT,
            5 => BlockType::CUESHEET,
            6 => BlockType::PICTURE,
            _ => BlockType::RESERVED
        }
    }
}