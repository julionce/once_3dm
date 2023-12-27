use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    point::Point3D,
    vector::Vector3D,
};

#[derive(Default, Deserialize)]
pub struct PlaneEquation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub d: f64,
}

#[derive(Default, Deserialize)]
pub struct Plane {
    pub origin: Point3D,
    pub x_axis: Vector3D,
    pub y_axis: Vector3D,
    pub z_axis: Vector3D,
    pub equation: PlaneEquation,
}
