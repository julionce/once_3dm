use crate::{
    bounding_box::BoundingBox,
    chunk,
    converters::U32IntoBool,
    curve::Curve,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    interval::Interval,
    line::Line,
};
use once_3dm_macros::Deserialize;

mod v1 {

    use super::*;

    #[derive(Default, Deserialize)]
    pub struct RevSurface {
        pub axis: Line,
        pub angle: Interval,
        pub bounding_box: BoundingBox,
        #[underlying_type(U32IntoBool)]
        pub transposed: bool,
        pub curve: Option<Curve>,
    }
}

mod v2 {

    use super::*;

    #[derive(Default, Deserialize)]
    pub struct RevSurface {
        pub axis: Line,
        pub angle: Interval,
        pub angle_parameter: Interval,
        pub bounding_box: BoundingBox,
        #[underlying_type(U32IntoBool)]
        pub transposed: bool,
        pub curve: Option<Curve>,
    }
}

#[derive(Default)]
pub enum RevSurface {
    #[default]
    Empty,
    V1(v1::RevSurface),
    V2(v2::RevSurface),
}

impl<V> Deserialize<V> for RevSurface
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let mut surface = RevSurface::default();
        let chunk_version = deserialize!(chunk::ShortVersion, V, ostream, "chunk_version");
        match chunk_version.major() {
            1 => {
                surface = RevSurface::V1(deserialize!(v1::RevSurface, V, ostream, "v1 surface"));
            }
            2 => {
                surface = RevSurface::V2(deserialize!(v2::RevSurface, V, ostream, "v2 surface"));
            }
            _ => {}
        }
        Ok(surface)
    }
}
