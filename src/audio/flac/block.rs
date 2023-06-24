use std::{ops::Deref, collections::HashMap, error::Error};
use super::meta::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    StreamInfo,
    Application,
    Padding,
    Seektable,
    VorbisComment,
    Cuesheet,
    Picture,
    Reserved
}

#[derive(Debug)]
pub enum BlockData {
    StreamInfo(StreamInfo),
    Application,
    Padding,
    Seektable,
    VorbisComment(VorbisComment),
    Cuesheet,
    Picture(Picture),
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

// pub struct BlockData(InnerBlockData);
// impl Deref for BlockData {
//     type Target = InnerBlockData;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

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
    pub fn parse(buf: Vec<u8>, block_type: BlockType) -> Result<BlockData, Box<dyn Error>> {
        let block = match block_type {
            BlockType::StreamInfo =>        BlockData::_parse_stream_info(buf),
            BlockType::VorbisComment =>     BlockData::_parse_vorbis_comment(buf),
            BlockType::Picture =>           BlockData::_parse_picture(buf),
            // BlockType::Padding =>           BlockData::
            _ => {
                // println!()
                todo!()
            }
        };

        block
    }

    fn _parse_stream_info(buf: Vec<u8>) -> Result<BlockData, Box<dyn Error>> {
        println!("==== StreamInfo ====");
        let min_block_size = u16::from_be_bytes(buf[0..2].try_into()?);
        println!("Min Block Size: {min_block_size}");
        let max_block_size = u16::from_be_bytes(buf[2..4].try_into()?);
        println!("Max Block Size: {max_block_size}");

        let mut min_frame_buf = [0u8; 4];
        min_frame_buf[1..].copy_from_slice(&buf[4..7]);
        let min_frame_size = u32::from_be_bytes(min_frame_buf);
        println!("Min Frame Size: {min_frame_size}");

        let mut max_frame_buf = [0u8; 4];
        max_frame_buf[1..].copy_from_slice(&buf[7..10]);
        let max_frame_size = u32::from_be_bytes(max_frame_buf);
        println!("Max Frame Size: {max_frame_size}");

        let sample_rate_part = u16::from_be_bytes(buf[10..12].try_into()?);
        let sample_ch_bps_part = u8::from_be_bytes([buf[12]]);
        let sample_rate = ((sample_rate_part as u32) << 4) | (sample_ch_bps_part as u32 >> 4);
        println!("Sample Rate: {sample_rate}");

        let channels = (sample_ch_bps_part >> 1) & 0b0000111;
        println!("Channels: {channels}");

        let bps_samples_part = u8::from_be_bytes([buf[13]]);
        let bits_per_sample = (((sample_ch_bps_part & 1) << 4) | bps_samples_part >> 4) + 1;
        println!("Bits Per Sample: {bits_per_sample}");

        let samples_part = bps_samples_part & 0b00001111;
        let samples_part_other = u32::from_be_bytes(buf[14..18].try_into()?);
        let samples = ((samples_part as u64) << 32) | (samples_part_other as u64);
        println!("Total Samples: {samples}");

        let md5 = buf[18..34].to_vec();
        println!("md5: {:#x?}", md5.clone());
        // dbg!(min_block_size, max_block_size, min_frame_size, max_frame_size, sample_rate, channels, bits_per_sample, samples, md5);

        Ok(BlockData::StreamInfo(StreamInfo {
            min_block_size,
            max_block_size,
            min_frame_size,
            max_frame_size,
            sample_rate,
            channels,
            bits_per_sample,
            samples,
            md5
        }))
    }

    fn _parse_vorbis_comment(buf: Vec<u8>) -> Result<BlockData, Box<dyn Error>> {
        println!("==== Vorbis Comment ====");
        let vendor_length = u32::from_le_bytes(buf[0..4].try_into()?);
        println!("Vendor Len: {vendor_length}");

        let vendor_string = String::from_utf8(buf[4..(vendor_length + 4) as usize].to_vec())?;
        println!("Vendor: {}", vendor_string.clone());
        let mut off = (vendor_length + 4) as usize;
        
        let comments_len = u32::from_le_bytes(buf[off..off + 4].try_into()?);
        println!("Comments: {comments_len}");
        off += 4;
        
        let mut comments = HashMap::new();
        for _ in 0..comments_len as usize {
            let comment_len = u32::from_le_bytes(buf[off..off + 4].try_into()?) as usize;
            let comment_string = String::from_utf8(buf[off + 4..off + 4 + comment_len].to_vec())?;
            println!("Comment: {comment_len} {comment_string}");
        
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
        
        Ok(BlockData::VorbisComment(VorbisComment { 
            vendor_string, 
            comments 
        }))
    }

    fn _parse_picture(buf: Vec<u8>) -> Result<BlockData, Box<dyn Error>> {
        println!("==== Picture ====");
        let picture_type: PictureType = u32::from_be_bytes(buf[0..4].try_into()?).into();
        println!("Picture Type: {picture_type:?}");

        let mime_type_len = u32::from_be_bytes(buf[4..8].try_into()?) as usize;
        println!("MIME Type Len: {mime_type_len}");

        let mime_type = String::from_utf8(buf[8..8 + mime_type_len].to_vec())?;
        println!("MIME Type: {}", mime_type.clone());
        let mut off = 8 + mime_type_len;

        let desc_len = u32::from_be_bytes(buf[off..off + 4].try_into()?) as usize;
        println!("Description Len: {desc_len}");
        off += 4;

        let description = String::from_utf8(buf[off..off + desc_len].to_vec())?;
        println!("Description: {}", description.clone());
        off += desc_len;

        let width = u32::from_be_bytes(buf[off..off + 4].try_into()?);
        off += 4;
        let height = u32::from_be_bytes(buf[off..off + 4].try_into()?);
        println!("Size: {width} x {height}");
        off += 4;

        let colour_depth = u32::from_be_bytes(buf[off..off + 4].try_into()?);
        println!("Depth: {colour_depth}");
        off += 4;

        let colour_count = u32::from_be_bytes(buf[off..off + 4].try_into()?);
        println!("Colour Count: {colour_count}");
        off += 4;

        let picture_len = u32::from_be_bytes(buf[off..off + 4].try_into()?) as usize;
        println!("Picture Len: {picture_len}");
        off += 4;

        let buffer = buf[off..off + picture_len].to_vec();
        
        Ok(BlockData::Picture(Picture {
            picture_type,
            mime_type,
            description,
            width,
            height,
            colour_depth,
            colour_count,
            buffer
        }))
    }
}

impl From<u8> for BlockType {
    fn from(value: u8) -> Self {
        match value {
            0 => BlockType::StreamInfo,
            1 => BlockType::Padding,
            2 => BlockType::Application,
            3 => BlockType::Seektable,
            4 => BlockType::VorbisComment,
            5 => BlockType::Cuesheet,
            6 => BlockType::Picture,
            _ => BlockType::Reserved
        }
    }
}