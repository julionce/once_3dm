use num_enum::TryFromPrimitive;
use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk, ChunkInStream},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    nurbs_curve::NurbsCurve,
    rollback::Rollback,
    type_code::TypeCode,
    uuid::Uuid,
};

const NURBS_CURVE_ID: Uuid = Uuid {
    data1: 0x4ED7D4DD,
    data2: 0xE947,
    data3: 0x11d3,
    data4: [0xBF, 0xE5, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const NURBS_CURVE_TL_ID: Uuid = Uuid {
    data1: 0x5EAF1119,
    data2: 0x0B51,
    data3: 0x11D4,
    data4: [0xBF, 0xFE, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const NURBS_CURVE_OLD_ID: Uuid = Uuid {
    data1: 0x76A709D5,
    data2: 0x1550,
    data3: 0x11D4,
    data4: [0x80, 0x00, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const NURBS_SURFACE_ID: Uuid = Uuid {
    data1: 0x4ED7D4DE,
    data2: 0xE947,
    data3: 0x11d3,
    data4: [0xBF, 0xE5, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const NURBS_SURFACE_TL_ID: Uuid = Uuid {
    data1: 0x4760C817,
    data2: 0x0BE3,
    data3: 0x11D4,
    data4: [0xBF, 0xFE, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const NURBS_SURFACE_OLD_ID: Uuid = Uuid {
    data1: 0xFA4FD4B5,
    data2: 0x1613,
    data3: 0x11D4,
    data4: [0x80, 0x00, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const POLY_CURVE_ID: Uuid = Uuid {
    data1: 0x4ED7D4E0,
    data2: 0xE947,
    data3: 0x11d3,
    data4: [0xBF, 0xE5, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const POLY_CURVE_OLD_ID: Uuid = Uuid {
    data1: 0xEF638317,
    data2: 0x154B,
    data3: 0x11D4,
    data4: [0x80, 0x00, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const BREP_ID: Uuid = Uuid {
    data1: 0x60B5DBC5,
    data2: 0xE660,
    data3: 0x11d3,
    data4: [0xBF, 0xE4, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const BREP_TL_ID: Uuid = Uuid {
    data1: 0xF06FC243,
    data2: 0xA32A,
    data3: 0x4608,
    data4: [0x9D, 0xD8, 0xA7, 0xD2, 0xC4, 0xCE, 0x2A, 0x36],
};

const BREP_OLD_ID: Uuid = Uuid {
    data1: 0x2D4CFEDB,
    data2: 0x3E2A,
    data3: 0x11D4,
    data4: [0x80, 0x0E, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const TRIMMED_SURFACE_OLD_ID: Uuid = Uuid {
    data1: 0x0705FDEF,
    data2: 0x3E2A,
    data3: 0x11D4,
    data4: [0x80, 0x0E, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const REV_SURFACE_ID: Uuid = Uuid {
    data1: 0xA16220D3,
    data2: 0x163B,
    data3: 0x11d4,
    data4: [0x80, 0x00, 0x00, 0x10, 0x83, 0x01, 0x22, 0xF0],
};

const REP_SURFACE_TL_ID: Uuid = Uuid {
    data1: 0xA8401B6,
    data2: 0x4D34,
    data3: 0x4B99,
    data4: [0x86, 0x15, 0x1B, 0x4E, 0x72, 0x3D, 0xC4, 0xE5],
};

const SUM_SURFACE_ID: Uuid = Uuid {
    data1: 0xC4CD5359,
    data2: 0x446D,
    data3: 0x4690,
    data4: [0x9F, 0xF5, 0x29, 0x05, 0x97, 0x32, 0x47, 0x2B],
};

const SUM_SURFACE_TL_ID: Uuid = Uuid {
    data1: 0x665F6331,
    data2: 0x2A66,
    data3: 0x4CCE,
    data4: [0x81, 0xD0, 0xB5, 0xEE, 0xBD, 0x9B, 0x54, 0x17],
};

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum Kind {
    NurbsCurve,
    NurbsSurface,
    PolyCurve,
    BRep,
    RevSurface,
    SumSurface,
}

impl TryInto<Kind> for Uuid {
    type Error = Error;

    fn try_into(self) -> Result<Kind, Self::Error> {
        match self {
            NURBS_CURVE_ID | NURBS_CURVE_TL_ID | NURBS_CURVE_OLD_ID => Ok(Kind::NurbsCurve),
            NURBS_SURFACE_ID | NURBS_SURFACE_TL_ID | NURBS_SURFACE_OLD_ID => Ok(Kind::NurbsSurface),
            POLY_CURVE_ID | POLY_CURVE_OLD_ID => Ok(Kind::PolyCurve),
            BREP_ID | BREP_TL_ID | BREP_OLD_ID | TRIMMED_SURFACE_OLD_ID => Ok(Kind::BRep),
            REV_SURFACE_ID | REP_SURFACE_TL_ID => Ok(Kind::RevSurface),
            SUM_SURFACE_ID | SUM_SURFACE_TL_ID => Ok(Kind::SumSurface),
            _ => Err(Error::Simple(ErrorKind::UnknownObjectId)),
        }
    }
}

#[derive(Default)]
pub enum Data {
    #[default]
    Empty,
    NurbsCurve(NurbsCurve),
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
