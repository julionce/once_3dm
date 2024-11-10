use crate::{
    bounding_box::BoundingBox,
    chunk::{self},
    curve::Curve,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    sequence::Sequence,
};

#[derive(Default)]
pub struct PolyCurve {
    pub segments_count: u32,
    pub bounding_box: BoundingBox,
    pub segment_params: Vec<f64>,
    pub segments: Vec<Curve>,
}

impl<V> Deserialize<V> for PolyCurve
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let mut curve = PolyCurve::default();
        let _version = deserialize!(chunk::ShortVersion, V, stream, "version");
        curve.segments_count = deserialize!(u32, V, stream, "segments_count");
        deserialize!(u32, V, stream, "reserved_1");
        deserialize!(u32, V, stream, "reserved_2");
        curve.bounding_box = deserialize!(BoundingBox, V, stream, "bounding_box");
        curve.segment_params = deserialize!(Sequence<u32, f64>, V, stream, "segment_params").into();
        for _ in 0..curve.segments_count {
            curve
                .segments
                .push(deserialize!(Curve, V, stream, "segment"));
        }
        // TODO: remove fuzz and nesting.
        Ok(curve)
    }
}
