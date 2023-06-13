use std::fs::File;

pub mod audio;

fn main() {
    audio::flac::Flac::load(
        File::open("test_files/1.flac").unwrap().into()
    );
}
