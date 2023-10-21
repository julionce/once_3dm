use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
