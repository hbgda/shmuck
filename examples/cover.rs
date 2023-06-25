use std::fs::File;

use shmuck::flac::{Flac, block::{BlockType, BlockData}, meta::Picture};

fn main() {
    let flac = Flac::new(
        File::open("test_files/2.flac").unwrap().into()
    ).unwrap();

    let BlockData::Picture(cover) = flac.get_block(BlockType::Picture)
        .unwrap()
        .data else { panic!() };

    std::fs::write("cover.jpg", cover.buffer).unwrap()
}