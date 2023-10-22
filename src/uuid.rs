use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize, PartialEq, Eq)]
pub struct Uuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
