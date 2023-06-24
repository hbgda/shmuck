use std::fs::File;

use crate::audio::flac::block::BlockType;

pub mod audio;

fn main() {
    let mut flac = audio::flac::Flac::new(
        File::open("test_files/2.flac").unwrap().into()
    ).expect("Failed to read??");


    dbg!(flac.get_block(BlockType::StreamInfo));
}
