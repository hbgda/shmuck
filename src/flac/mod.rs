pub mod block;
pub mod meta;

use std::{io::Read, error::Error, fs, ops::{Deref, DerefMut}, collections::HashMap};

use crate::BasicMetadata;

use self::{block::{BlockHeader, BlockData, MetadataBlock, BlockType}, meta::VorbisComment};

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
impl FlacStream {
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let mut buf = vec![0u8; size];
        self.0.read_exact(&mut buf).unwrap();
        buf
    }
}

pub struct Flac {
    pub meta: BasicMetadata,
    blocks: HashMap<BlockType, MetadataBlock>
}

impl Flac {
    pub fn new(mut stream: FlacStream) -> Result<Flac, Box<dyn Error>> {
        if stream.read(4) != [0x66, 0x4c, 0x61, 0x43] {
            return Err("File not FLAC.".into())
        }

        let mut meta = BasicMetadata { title: "".into(), artist: "".into(), album: None };
        let mut blocks = HashMap::new();
        loop {
            let block = Flac::parse_block(&mut stream)?;
            if let BlockData::VorbisComment(v) = block.data.clone() {
                if let Some(_meta) = v.into_meta() {
                    meta = _meta;
                }
            }
            blocks.insert(block.header.block_type, block.clone());
            if block.header.last_block {
                break;
            }
        }

        Ok(Flac { meta, blocks })
    }

    // pub fn get_block(&mut self, block_type: BlockType) -> Option<MetadataBlock> {
    //     if let Some(block) = self.blocks.iter().find(|b| b.header.block_type == block_type) {
    //         return Some(block.clone());
    //     }
    //     let index = self.block_locs.get(&block_type)?;
    //     if let Err(_) = self.stream.seek(SeekFrom::Start(*index as u64)) {
    //         return None;
    //     }
    //     let block = Flac::parse_block(&mut self.stream);
    //     if let Err(_) = block {
    //         return None;
    //     }
    //     let block = block.unwrap();
    //     self.blocks.push(block.clone());
    //     return Some(block);
    // }

    pub fn get_block(&self, block_type: BlockType) -> Option<MetadataBlock> {
        Some(self.blocks.get(&block_type)?.clone())
    }

    fn parse_block(stream: &mut FlacStream) -> Result<MetadataBlock, Box<dyn Error>> {
        let header_buf = stream.read(4);
        let header = BlockHeader::parse(header_buf);
        if header.block_type == BlockType::Padding {
            _ = stream.read(header.len as usize);
            return Ok(MetadataBlock { header, data: BlockData::Padding })
        }
        
        let data_buf = stream.read(header.len as usize);
        let data = BlockData::parse(data_buf, header.block_type)?;

        Ok(MetadataBlock {
            header, data
        })
    }
}