use once_3dm_macros::Deserialize;

use crate::{
    bounding_box::BoundingBox,
    chunk::{self},
    color::Color,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    plane::Plane,
    sequence::Sequence,
    vector::Vector3D,
};

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Deserialize, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Deserialize)]
pub struct Point4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct PointCloud {
    #[underlying_type(Sequence<u32, Point3D>)]
    pub points: Vec<Point3D>,
    pub plane: Plane,
    pub bounding_box: BoundingBox,
    pub flags: u32,
    #[if_minor_version(Ge(1))]
    #[underlying_type(Sequence<u32, Vector3D>)]
    pub point_normals: Vec<Vector3D>,
    #[if_minor_version(Ge(1))]
    #[underlying_type(Sequence<u32, Color>)]
    pub point_colors: Vec<Color>,
    #[if_minor_version(Ge(2))]
    #[underlying_type(Sequence<u32, f64>)]
    pub point_values: Vec<f64>,
}
