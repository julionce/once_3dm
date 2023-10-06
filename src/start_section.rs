use std::io::SeekFrom;

use once_io::OStream;

use crate::{
    chunk::Begin,
    deserialize::{Deserialize, V1},
    error::ErrorStack,
    typecode::{self},
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
            let chunk_begin = <Begin as Deserialize<V1>>::deserialize(ostream)?;
            match chunk_begin.typecode {
                typecode::SUMMARY
                | typecode::BITMAPPREVIEW
                | typecode::UNIT_AND_TOLERANCES
                | typecode::VIEWPORT
                | typecode::LAYER
                | typecode::RENDERMESHPARAMS
                | typecode::CURRENTLAYER
                | typecode::ANNOTATION_SETTINGS
                | typecode::NOTES
                | typecode::NAMED_CPLANE
                | typecode::NAMED_VIEW => {
                    ostream
                        .seek(SeekFrom::Current(chunk_begin.length as i64))
                        .unwrap();
                }
                _ => {
                    if typecode::TABLE == chunk_begin.typecode & 0xFFFF0000 {
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
