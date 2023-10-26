use once_3dm_macros::Deserialize;

use crate::{
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
