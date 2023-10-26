use once_3dm_macros::Deserialize;

use crate::{
    chunk, deserialize,
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

#[derive(Default, Deserialize)]
#[with_version(short)]
pub struct PolyLineCurve {
    pub poly_line: PolyLine,
    #[underlying_type(Sequence<u32, f64>)]
    pub parameters: Vec<f64>,
    pub dimension: u32,
}
