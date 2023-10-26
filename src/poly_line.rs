use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    point::Point3D,
    sequence::Sequence,
};

#[derive(Default, Deserialize)]
pub struct PolyLine {
    #[underlying_type(Sequence<u32, Point3D>)]
    pub points: Vec<Point3D>,
}
