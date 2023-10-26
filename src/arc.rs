use once_3dm_macros::Deserialize;

use crate::{
    chunk,
    circle::Circle,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    interval::Interval,
};

#[derive(Default, Deserialize)]
pub struct Arc {
    pub circle: Circle,
    pub interval: Interval,
}

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct ArcCurve {
    pub arc: Arc,
    pub interval: Interval,
    pub dimension: u32,
}
