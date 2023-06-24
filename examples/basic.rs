use std::fs::File;

use shmuck::flac::{Flac, block::BlockType};

fn main() {
    let mut flac = Flac::new(
        File::open("test_files/2.flac").unwrap().into()
    ).expect("Failed to read??");


    dbg!(flac.get_block(BlockType::StreamInfo));
    println!("Title: {}", flac.title().unwrap())
}
