use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    plane::Plane,
    point::Point3D,
};

#[derive(Default, Deserialize)]
pub struct Circle {
    pub plane: Plane,
    pub radius: f64,
    _scratch: [Point3D; 3],
}
