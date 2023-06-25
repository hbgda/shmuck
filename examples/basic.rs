use std::fs::File;

use shmuck::{flac::{Flac, block::BlockType}, Metadata};

fn main() {
    let flac = Flac::new(
        File::open("test_files/2.flac").unwrap().into()
    ).expect("Failed to read??");


    dbg!(flac.get_block(BlockType::StreamInfo));
    println!("Meta: {:?}", flac.metadata())
}
