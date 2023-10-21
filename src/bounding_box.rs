use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    point::Point,
};

#[derive(Default, Deserialize)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}
