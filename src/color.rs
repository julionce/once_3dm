use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default, Deserialize)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
