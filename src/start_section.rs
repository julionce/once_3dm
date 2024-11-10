use std::io::{Seek, SeekFrom};

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

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let backup_position = SeekFrom::Start(stream.stream_position().unwrap());
        let mut version = Version::V1;
        loop {
            let begin = deserialize!(chunk::Begin, V1, stream, "begin");
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
                    stream.seek(SeekFrom::Current(begin.length as i64)).unwrap();
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
            stream.seek(backup_position).unwrap();
        }
        Ok(StartSection { version })
    }
}
