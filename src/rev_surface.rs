use crate::{
    bounding_box::BoundingBox,
    chunk,
    converters::U32IntoBool,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    interval::Interval,
    line::Line,
    object,
};

#[derive(Default)]
pub struct RevSurface {
    pub axis: Line,
    pub angle: Interval,
    pub angle_parameter: Interval,
    pub bounding_box: BoundingBox,
    pub transposed: bool,
    pub has_curve: bool,
    pub curve: Box<object::Class>,
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
                surface.axis = deserialize!(Line, V, ostream, "axis");
                surface.angle = deserialize!(Interval, V, ostream, "angle");
                surface.bounding_box = deserialize!(BoundingBox, V, ostream, "bounding_box");
                surface.transposed = deserialize!(U32IntoBool, V, ostream, "transposed").into();
                surface.has_curve = deserialize!(bool, V, ostream, "has_curve");
                if surface.has_curve {
                    surface.curve = Box::new(deserialize!(object::Class, V, ostream, "curve"));
                }
            }
            2 => {
                surface.axis = deserialize!(Line, V, ostream, "axis");
                surface.angle = deserialize!(Interval, V, ostream, "angle");
                surface.angle_parameter = deserialize!(Interval, V, ostream, "angle_parameter");
                surface.bounding_box = deserialize!(BoundingBox, V, ostream, "bounding_box");
                surface.transposed = deserialize!(U32IntoBool, V, ostream, "transposed").into();
                surface.has_curve = deserialize!(bool, V, ostream, "has_curve");
                if surface.has_curve {
                    surface.curve = Box::new(deserialize!(object::Class, V, ostream, "curve"));
                }
            }
            _ => {}
        }
        Ok(surface)
    }
}
