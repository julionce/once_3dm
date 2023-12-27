use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    point::Point3D,
};

#[derive(Default, Deserialize)]
pub struct BoundingBox {
    pub min: Point3D,
    pub max: Point3D,
}
