use std::{ops::Deref, collections::HashMap};

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub enum InnerBlockData {
    STREAMINFO(StreamInfo),
    VORBIS_COMMENT(VorbisComment)
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
            BlockType::STREAMINFO =>        BlockData::_parse_stream_info(buf),
            BlockType::VORBIS_COMMENT =>    BlockData::_parse_vorbis_comment(buf),
            _ => {
                // println!()
                todo!()
            }
        };

        BlockData(block)
    }

    fn _parse_stream_info(buf: Vec<u8>) -> InnerBlockData {
        let min_block_size = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        let max_block_size = u16::from_be_bytes(buf[2..4].try_into().unwrap());

        let mut min_frame_buf = [0u8; 4];
        min_frame_buf[1..].copy_from_slice(&buf[4..7]);
        let min_frame_size = u32::from_be_bytes(min_frame_buf);
        
        let mut max_frame_buf = [0u8; 4];
        max_frame_buf[1..].copy_from_slice(&buf[7..10]);
        let max_frame_size = u32::from_be_bytes(max_frame_buf);

        let sample_rate_part = u16::from_be_bytes(buf[10..12].try_into().unwrap());
        let sample_ch_bps_part = u8::from_be_bytes([buf[12]]);
        let sample_rate = ((sample_rate_part as u32) << 4) | (sample_ch_bps_part as u32 >> 4);
        
        let channels = (sample_ch_bps_part >> 1) & 0b0000111;

        let bps_samples_part = u8::from_be_bytes([buf[13]]);
        let bits_per_sample = (((sample_ch_bps_part & 1) << 4) | bps_samples_part >> 4) + 1;

        let samples_part = bps_samples_part & 0b00001111;
        let samples_part_other = u32::from_be_bytes(buf[14..18].try_into().unwrap());
        let samples = ((samples_part as u64) << 32) | (samples_part_other as u64);

        let md5 = u128::from_be_bytes(buf[18..34].try_into().unwrap());
        // dbg!(min_block_size, max_block_size, min_frame_size, max_frame_size, sample_rate, channels, bits_per_sample, samples, md5);

        InnerBlockData::STREAMINFO(StreamInfo {
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bits_per_sample,
            samples,
            md5
        })
    }
    fn _parse_vorbis_comment(buf: Vec<u8>) -> InnerBlockData {
        let vendor_length = u32::from_le_bytes(buf[0..4].try_into().unwrap());
        // println!("{vendor_length}");
        let vendor_string = String::from_utf8(buf[4..(vendor_length + 4) as usize].to_vec()).unwrap();
        // println!("{vendor_string}");
        let mut off = (vendor_length + 4) as usize;
        let comments_len = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
        // println!("{comment_len}");
        off += 4;
        let mut comments = HashMap::new();
        for _ in 0..comments_len as usize {
            let comment_len = u32::from_le_bytes(buf[off..off + 4].try_into().unwrap()) as usize;
            let comment_string = String::from_utf8(buf[off + 4..off + 4 + comment_len].try_into().unwrap()).unwrap();
            // println!("{comment_len} {comment_string}");
            let parts: Vec<&str> = comment_string.split("=").collect();
            comments.entry(parts[0].to_string())
                .and_modify(|e: &mut Vec<String>| 
                    e.push(parts[1].to_string())
                )
                .or_insert(
                    Vec::from([parts[1].to_string()])
                );
            off += 4 + comment_len;
        }
        
        dbg!(InnerBlockData::VORBIS_COMMENT(VorbisComment { 
            vendor_string , 
            comments 
        }))
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