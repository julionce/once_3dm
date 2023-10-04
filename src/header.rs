use once_io::OStream;

use crate::{
    comment::Comment,
    deserialize::{Deserialize, V1, V2, V3, V4, V50, V60, V70},
    start_section::StartSection,
    version::Version,
};

pub struct Header {
    pub version: Version,
    pub comment: String,
}

impl Header {
    const BEGIN: &'static [u8] = "3D Geometry File Format ".as_bytes();
}

impl Deserialize<V1> for Header {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let mut buf = [0; Header::BEGIN.len()];
        match ostream.read_exact(&mut buf) {
            Ok(()) => {
                if Header::BEGIN != buf {
                    return Err("invalid header".to_string());
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        let mut version = <Version as Deserialize<V1>>::deserialize(ostream)?;
        let start_section = match version {
            Version::V1 => <StartSection as Deserialize<V1>>::deserialize(ostream)?,
            _ => StartSection { version },
        };
        version = start_section.version;
        let comment = match version {
            Version::V1 => <Comment as Deserialize<V1>>::deserialize(ostream)?,
            Version::V2 => <Comment as Deserialize<V2>>::deserialize(ostream)?,
            Version::V3 => <Comment as Deserialize<V3>>::deserialize(ostream)?,
            Version::V4 => <Comment as Deserialize<V4>>::deserialize(ostream)?,
            Version::V50 => <Comment as Deserialize<V50>>::deserialize(ostream)?,
            Version::V60 => <Comment as Deserialize<V60>>::deserialize(ostream)?,
            Version::V70 => <Comment as Deserialize<V70>>::deserialize(ostream)?,
        };
        Ok(Header {
            version,
            comment: comment.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Cursor};

    use super::*;

    #[test]
    fn deserialize_ok() {
        let mut ostream = File::open("resource/v1/three_points.3dm").unwrap();
        assert!(Header::deserialize(&mut ostream).is_ok());
    }

    #[test]
    fn deserialize_invalid_begin_not_match() {
        let data = "4D Geometry File Format ".as_bytes();
        let mut ostream = Cursor::new(data);
        assert!(Header::deserialize(&mut ostream).is_err());
    }

    #[test]
    fn deserialize_invalid_begin_io_error() {
        let data = "3D Geometry File Format".as_bytes();
        let mut ostream = Cursor::new(data);
        assert!(Header::deserialize(&mut ostream).is_err());
    }
}
