use std::io::SeekFrom;

use once_io::OStream;

use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, V1},
    error::ErrorStack,
    type_code::TypeCode,
    version::Version,
};

pub struct StartSection {
    pub version: Version,
}

impl Deserialize<V1> for StartSection {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let backup_position = SeekFrom::Start(ostream.stream_position().unwrap());
        let mut version = Version::V1;
        loop {
            let begin = deserialize!(chunk::Begin, V1, ostream, "begin");
            match begin.type_code {
                TypeCode::Summary
                | TypeCode::BitmapPreview
                | TypeCode::UnitsAndTolerances
                | TypeCode::Viewport
                | TypeCode::Layer
                | TypeCode::RenderMeshParams
                | TypeCode::CurrentLayer
                | TypeCode::AnnotationSettings
                | TypeCode::Notes
                | TypeCode::NamedCPlane
                | TypeCode::NamedView => {
                    ostream
                        .seek(SeekFrom::Current(begin.length as i64))
                        .unwrap();
                }
                _ => {
                    if TypeCode::Table as u32 == begin.type_code as u32 & 0xFFFF0000 {
                        version = Version::V2
                    }
                    break;
                }
            }
        }
        if Version::V1 == version {
            ostream.seek(backup_position).unwrap();
        }
        Ok(StartSection { version })
    }
}
