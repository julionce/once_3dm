use once_io::OStream;

use crate::{
    bounding_box::BoundingBox,
    chunk::{self, ChunkInStream},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    object,
    sequence::Sequence,
    type_code::TypeCode,
};

#[derive(Default)]
pub struct PolyCurve {
    pub segments_count: u32,
    pub bounding_box: BoundingBox,
    pub segment_params: Vec<f64>,
    pub segments: Vec<object::Class>,
}

impl<V> Deserialize<V> for PolyCurve
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let mut curve = PolyCurve::default();
        let _version = deserialize!(chunk::ShortVersion, V, ostream, "version");
        curve.segments_count = deserialize!(u32, V, ostream, "segments_count");
        deserialize!(u32, V, ostream, "reserved_1");
        deserialize!(u32, V, ostream, "reserved_2");
        curve.bounding_box = deserialize!(BoundingBox, V, ostream, "bounding_box");
        curve.segment_params =
            deserialize!(Sequence<u32, f64>, V, ostream, "segment_params").into();
        for _ in 0..curve.segments_count {
            let object = deserialize!(object::Class, V, ostream, "object");
            match object.inner.data {
                object::Data::ArcCurve(_)
                | object::Data::NurbsCurve(_)
                | object::Data::LineCurve(_)
                | object::Data::PolyLineCurve(_)
                | object::Data::PolyCurve(_) => {
                    curve.segments.push(object);
                }
                _ => {
                    // TODO: return error
                }
            };
        }
        // TODO: remove fuzz and nesting.
        Ok(curve)
    }
}
