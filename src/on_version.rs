use once_3dm_macros::Deserialize;

use once_io::OStream;

use crate::deserialize::{Deserialize, FileVersion};

#[derive(Default, Deserialize)]
pub struct OnVersion {
    //TODO: process raw version
    pub raw_version: u32,
}
