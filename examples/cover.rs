use std::fs::File;

use shmuck::{flac::{Flac, block::{BlockType, BlockData}, meta::Picture}, Cover};

fn main() {
    let flac = Flac::new(
        File::open("test_files/2.flac").unwrap().into()
    ).unwrap();

    let cover = flac.cover().unwrap();

    std::fs::write("cover.jpg", cover.buffer).unwrap()
}