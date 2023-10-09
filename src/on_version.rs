use once_3dm_macros::Deserialize;

use crate::{
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize)]
pub struct OnVersion {
    //TODO: process raw version
    pub raw_version: u32,
}
