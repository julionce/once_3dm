use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
