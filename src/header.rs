use crate::{
    comment::Comment,
    deserialize,
    deserialize::{Deserialize, V1, V2, V3, V4, V50, V60, V70},
    error::{Error, ErrorKind, ErrorStack},
    start_section::StartSection,
    version::Version,
};

use std::io::Read;

pub struct Header {
    pub version: Version,
    pub comment: String,
}

impl Header {
    const BEGIN: &'static [u8] = "3D Geometry File Format ".as_bytes();
}

impl Deserialize<V1> for Header {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let mut buf = [0; Header::BEGIN.len()];
        match stream.read_exact(&mut buf) {
            Ok(()) => {
                if Header::BEGIN != buf {
                    return Err(ErrorStack::new(Error::Simple(ErrorKind::InvalidHeader)));
                }
            }
            Err(e) => return Err(ErrorStack::new(Error::IoError(e))),
        }
        let mut version = deserialize!(Version, V1, stream, "version");
        let start_section = match version {
            Version::V1 => deserialize!(StartSection, V1, stream, "start_section"),
            _ => StartSection { version },
        };
        version = start_section.version;
        let comment = match version {
            Version::V1 => deserialize!(Comment, V1, stream, "comment"),
            Version::V2 => deserialize!(Comment, V2, stream, "comment"),
            Version::V3 => deserialize!(Comment, V3, stream, "comment"),
            Version::V4 => deserialize!(Comment, V4, stream, "comment"),
            Version::V50 => deserialize!(Comment, V50, stream, "comment"),
            Version::V60 => deserialize!(Comment, V60, stream, "comment"),
            Version::V70 => deserialize!(Comment, V70, stream, "comment"),
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
        let mut file = File::open("resource/v1/three_points.3dm").unwrap();
        let mut stream = once_io::Stream::new(&mut file);
        assert!(Header::deserialize(&mut stream).is_ok());
    }

    #[test]
    fn deserialize_invalid_begin_not_match() {
        let data = "4D Geometry File Format ".as_bytes();
        let mut cursor = Cursor::new(data);
        let mut stream = once_io::Stream::new(&mut cursor);
        assert!(Header::deserialize(&mut stream).is_err());
    }

    #[test]
    fn deserialize_invalid_begin_io_error() {
        let data = "3D Geometry File Format".as_bytes();
        let mut cursor = Cursor::new(data);
        let mut stream = once_io::Stream::new(&mut cursor);
        assert!(Header::deserialize(&mut stream).is_err());
    }
}
