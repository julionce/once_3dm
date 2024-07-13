use once_3dm_macros::Deserialize;

use crate::{
    chunk,
    converters::U32IntoBool,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize)]
#[with_chunk_version(short)]
#[if_major_version(Eq(1))]
pub struct MeshParameters {
    #[underlying_type(U32IntoBool)]
    pub compute_curvature: bool,
    #[underlying_type(U32IntoBool)]
    pub simple_planes: bool,
    #[underlying_type(U32IntoBool)]
    pub refine: bool,
    #[underlying_type(U32IntoBool)]
    pub jagged_seams: bool,
    #[padding(i32)]
    pub tolerance: f64,
    pub min_edge_lenght: f64,
    pub max_edge_length: f64,
    pub grid_aspect_ration: f64,
    pub refine_angle_radians: f64,
    #[padding(f64)]
    //TODO: review opennurbs logic
    pub face_type: u32,
    #[if_minor_version(Ge(1))]
    pub texture_range: u32,
    #[if_minor_version(Ge(2))]
    pub custom_settings: bool,
    #[if_minor_version(Ge(3))]
    pub mesher: u8,
    #[if_minor_version(Ge(4))]
    pub custom_settings_enabled: bool,
    // if_minor_version(Ge(5))
    // pub subd_density_parameters
}
