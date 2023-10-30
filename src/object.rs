use num_enum::TryFromPrimitive;
use once_3dm_macros::Deserialize;

use crate::{
    arc::ArcCurve,
    chunk::{self, Chunk, ChunkInStream},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    line::LineCurve,
    nurbs_curve::NurbsCurve,
    object_id::*,
    point::{Point, PointCloud},
    poly_line::PolyLineCurve,
    rollback::Rollback,
    type_code::TypeCode,
    uuid::Uuid,
};

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Kind {
    ArcCurve,
    BRep,
    LineCurve,
    NurbsCurve,
    NurbsSurface,
    Point,
    PointCloud,
    PolyCurve,
    PolyLineCurve,
    RevSurface,
    SumSurface,
    TextDot,
    TextDotObsoleteV2,
}

impl TryInto<Kind> for Uuid {
    type Error = Error;

    fn try_into(self) -> Result<Kind, Self::Error> {
        match self {
            NURBS_CURVE | NURBS_CURVE_TL | NURBS_CURVE_OLD => Ok(Kind::NurbsCurve),
            NURBS_SURFACE | NURBS_SURFACE_TL | NURBS_SURFACE_OLD => Ok(Kind::NurbsSurface),
            POLY_CURVE | POLY_CURVE_OLD => Ok(Kind::PolyCurve),
            BREP | BREP_TL | BREP_OLD | TRIMMED_SURFACE => Ok(Kind::BRep),
            REV_SURFACE | REP_SURFACE_TL => Ok(Kind::RevSurface),
            SUM_SURFACE | SUM_SURFACE_TL => Ok(Kind::SumSurface),
            POINT => Ok(Kind::Point),
            POINT_CLOUD => Ok(Kind::PointCloud),
            LINE_CURVE => Ok(Kind::LineCurve),
            ARC_CURVE => Ok(Kind::ArcCurve),
            POLY_LINE_CURVE => Ok(Kind::PolyLineCurve),
            TEXT_DOT => Ok(Kind::TextDot),
            OBSOLETE_V2_TEXT_DOT => Ok(Kind::TextDotObsoleteV2),
            _ => Err(Error::Simple(ErrorKind::UnknownObjectId)),
        }
    }
}

#[derive(Default)]
pub enum Data {
    #[default]
    Empty,
    ArcCurve(ArcCurve),
    LineCurve(LineCurve),
    NurbsCurve(NurbsCurve),
    Point(Point),
    PointCloud(PointCloud),
    PolyLineCurve(PolyLineCurve),
}

#[derive(Default)]
pub struct Class {
    pub uuid: Uuid,
    pub data: Data,
}

impl<V> Deserialize<V> for Class
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
        let mut class = Class::default();
        let uuid = deserialize!(
            Chunk::<{ TypeCode::OpenNurbsClassUuid as u32 }, Uuid>,
            V,
            ostream,
            "uuid"
        )
        .inner;
        class.data = match TryInto::<Kind>::try_into(uuid) {
            Ok(kind) => match kind {
                Kind::NurbsCurve => Data::NurbsCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, NurbsCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::Point => Data::Point(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, Point>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::PointCloud => Data::PointCloud(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, PointCloud>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::LineCurve => Data::LineCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, LineCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::ArcCurve => Data::ArcCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, ArcCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                Kind::PolyLineCurve => Data::PolyLineCurve(
                    deserialize!(
                        ChunkInStream::<{ TypeCode::OpenNurbsClassData as u32 }, PolyLineCurve>,
                        V,
                        ostream
                    )?
                    .inner,
                ),
                _ => Data::default(),
            },
            Err(e) => {
                return Err(ErrorStack::new(e));
            }
        };
        Ok(class)
    }
}

#[derive(Default, Deserialize)]
pub struct Record {
    #[in_chunk(ObjectRecordType)]
    _empty: (),
    #[in_chunk(OpenNurbsClass)]
    pub class: Class,
}

#[derive(Default)]
pub struct Table {
    pub records: Vec<Record>,
}

impl<V> Deserialize<V> for Table
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
        let mut records = vec![];
        loop {
            let type_code = deserialize!(Rollback<TypeCode>, V, ostream, "type_code").inner;
            match type_code {
                TypeCode::ObjectRecord => {
                    records.push(
                        deserialize!(
                            Chunk::<{ TypeCode::ObjectRecord as u32 }, Record>,
                            V,
                            ostream,
                            "record"
                        )
                        .inner,
                    );
                }
                TypeCode::EndOfTable => {
                    break;
                }
                _ => {
                    return Err(ErrorStack::new(Error::Simple(
                        ErrorKind::InvalidChunkTypeCode,
                    )));
                }
            };
        }
        Ok(Self { records })
    }
}
