pub mod block;
pub mod meta;

use std::{io::{Read, self}, error::Error, fs, ops::{Deref, DerefMut}};

use self::block::{BlockHeader, BlockData, MetadataBlock};


pub struct Flac {
    buffer: Vec<u8>
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
impl FlacStream {
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        let mut buf = vec![0u8; size];
        self.0.read_exact(&mut buf).unwrap();
        buf
    }
}

impl Flac {
    pub fn load(mut stream: FlacStream) -> Result<Flac, Box<dyn Error>> {
        let magic_buf = stream.read(4);
        let magic_buf = dbg!(magic_buf);
        if magic_buf != [0x66, 0x4C, 0x61, 0x43] {
            return Err("Provided file is not FLAC.".into());
        }
        println!("Is FLAC");

        Flac::parse_block(&mut stream);
        Flac::parse_block(&mut stream);
        Flac::parse_block(&mut stream);
        todo!()
    }

    fn parse_block(stream: &mut FlacStream) -> Result<MetadataBlock, Box<dyn Error>> {
        let header_buf = stream.read(4);
        let header = BlockHeader::parse(header_buf);
        
        let data_buf = stream.read(header.len as usize);
        let data = BlockData::parse(data_buf, header.block_type);

        Ok(MetadataBlock {
            header, data
        })
    }

    fn load_metadata(&mut self) {

    }
}

