use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    point::Point3D,
};

#[derive(Default, Deserialize)]
pub struct Line {
    pub from: Point3D,
    pub to: Point3D,
}
