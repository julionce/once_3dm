use once_3dm_macros::Deserialize;

use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    interval::Interval,
    point::Point3D,
};

#[derive(Default, Deserialize)]
pub struct Line {
    pub from: Point3D,
    pub to: Point3D,
}

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct LineCurve {
    pub line: Line,
    pub interval: Interval,
    pub dimension: u32,
}
