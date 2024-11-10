use crate::{
    bounding_box::BoundingBox,
    chunk,
    converters::U32IntoBool,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
};

#[derive(Default)]
pub struct NurbsCurve {
    pub dimension: u32,
    pub is_rational: bool,
    pub order: u32,
    pub control_vertex_count: u32,
    pub bounding_box: BoundingBox,
    pub knots: Vec<f64>,
    pub control_vertexes: Vec<Vec<f64>>,
    pub sub_d_friendly_tag: bool,
}

impl<V> Deserialize<V> for NurbsCurve
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let mut curve = NurbsCurve::default();
        let version = deserialize!(chunk::ShortVersion, V, stream, "version");
        if 1 == version.major() {
            curve.dimension = deserialize!(u32, V, stream, "dimension");
            curve.is_rational = deserialize!(U32IntoBool, V, stream, "is_rational").into();
            curve.order = deserialize!(u32, V, stream, "order");
            curve.control_vertex_count = deserialize!(u32, V, stream, "control_vertex_count");
            deserialize!(u32, V, stream)?;
            deserialize!(u32, V, stream)?;
            curve.bounding_box = deserialize!(BoundingBox, V, stream, "bounding_box");
            let knots_count = deserialize!(u32, V, stream, "knots_count");
            if (i32::MAX as u32) < knots_count {
                return Err(ErrorStack::new(Error::Simple(ErrorKind::InvalidKnotsCount)));
            }
            let expected_knots_count = match curve.order.checked_add(curve.control_vertex_count) {
                Some(count) => match count.checked_sub(2) {
                    Some(count) => Some(count),
                    None => None,
                },
                None => None,
            };
            match expected_knots_count {
                Some(count) if 0 == knots_count || count == knots_count => {}
                _ => {
                    return Err(ErrorStack::new(Error::Simple(ErrorKind::InvalidKnotsCount)));
                }
            }
            for _ in 0..knots_count {
                curve.knots.push(deserialize!(f64, V, stream, "knot"));
            }
            let _count = deserialize!(u32, V, stream, "count");
            let control_vertexes_size = match (curve.dimension, curve.is_rational) {
                (0, _) => 0u32,
                (d, true) => d + 1,
                (d, false) => d,
            };
            for _ in 0..curve.control_vertex_count {
                let mut row: Vec<f64> = vec![];
                for _ in 0..control_vertexes_size {
                    row.push(deserialize!(f64, V, stream, "vertex"));
                }
                curve.control_vertexes.push(row);
            }
            if 1 <= version.minor() {
                curve.sub_d_friendly_tag = deserialize!(bool, V, stream, "sub_d_friendly_tag");
            }
        }
        Ok(curve)
    }
}
