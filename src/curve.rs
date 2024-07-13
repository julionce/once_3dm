use once_3dm_macros::Deserialize;

use crate::{
    arc::ArcCurve,
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    line::LineCurve,
    nurbs_curve::NurbsCurve,
    object,
    poly_curve::PolyCurve,
    poly_line::PolyLineCurve,
    sequence::Sequence,
    type_code::TypeCode,
};

pub enum Curve {
    ArcCurve(ArcCurve),
    LineCurve(LineCurve),
    NurbsCurve(NurbsCurve),
    PolyCurve(PolyCurve),
    PolyLineCurve(PolyLineCurve),
}

impl<V> Deserialize<V> for Curve
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
        let object = deserialize!(object::Class, V, ostream, "object");
        match object.inner.data {
            object::Data::ArcCurve(arc) => Ok(Curve::ArcCurve(arc)),
            object::Data::LineCurve(line) => Ok(Curve::LineCurve(line)),
            object::Data::NurbsCurve(nurbs) => Ok(Curve::NurbsCurve(nurbs)),
            object::Data::PolyCurve(poly) => Ok(Curve::PolyCurve(poly)),
            object::Data::PolyLineCurve(poly_line) => Ok(Curve::PolyLineCurve(poly_line)),
            _ => Err(ErrorStack::new(Error::Simple(ErrorKind::ObjectIsNotACurve))),
        }
    }
}

#[derive(Default, Deserialize)]
#[with_chunk_version(short)]
#[if_major_version(Eq(1))]
pub struct ArrayV1 {
    #[underlying_type(Sequence<u32, Curve>)]
    pub data: Vec<Curve>,
}

#[derive(Default, Deserialize)]
pub struct Array {
    #[in_chunk(AnonymousChunk)]
    pub v1: ArrayV1,
}
