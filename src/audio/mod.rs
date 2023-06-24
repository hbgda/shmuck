use std::sync::Arc;

pub mod util;
pub mod flac;

// pub fn load<T: Format, S: Source>(source: S) -> T { }

pub trait Metadata {
    fn title(&self) -> Option<String>;
    fn artist(&self) -> Option<String>;
    fn cover(&self) -> Option<Arc<[u8]>>;
    // the rest to come i suppose
}