use std::io::{Seek, SeekFrom};

use once_3dm_macros::Deserialize;
use once_io::OStream;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::{Error, ErrorKind, ErrorStack},
    typecode::{self, Typecode},
};

pub struct Begin {
    pub typecode: Typecode,
    pub length: u64,
}

impl Deserialize<V1> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = deserialize!(Typecode, V1, ostream, "typecode");
        let is_unsigned = 0 == (typecode::SHORT & typecode)
            || typecode::RGB == typecode
            || typecode::RGBDISPLAY == typecode
            || typecode::PROPERTIES_OPENNURBS_VERSION == typecode
            || typecode::OBJECT_RECORD_TYPE == typecode;
        let value = if is_unsigned {
            deserialize!(u32, V1, ostream, "length") as i64
        } else {
            deserialize!(i32, V1, ostream, "length") as i64
        };
        let is_long = (0 == typecode & typecode::SHORT) && (0 != typecode) && (0 < value);
        let length = if is_long { value as u64 } else { 0u64 };
        Ok(Begin { typecode, length })
    }
}

//TODO: check is_long and length.
impl Deserialize<V2> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = deserialize!(Typecode, V2, ostream, "typecode");
        if typecode::PROPERTIES_OPENNURBS_VERSION == typecode {
            Ok(Begin {
                typecode,
                length: 4u64,
            })
        } else {
            let is_unsigned = 0 == (typecode::SHORT & typecode)
                || typecode::RGB == typecode
                || typecode::RGBDISPLAY == typecode
                || typecode::OBJECT_RECORD_TYPE == typecode;
            let value = if is_unsigned {
                deserialize!(u32, V2, ostream, "length") as i64
            } else {
                deserialize!(i32, V2, ostream, "length") as i64
            };
            let is_long = (0 == typecode & typecode::SHORT) && (0 < value);
            let length = if is_long { value as u64 } else { 0u64 };
            Ok(Begin { typecode, length })
        }
    }
}

impl Deserialize<V3> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        deserialize!(Begin, V2, ostream)
    }
}

impl Deserialize<V4> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        deserialize!(Begin, V2, ostream)
    }
}

//TODO: check is_long and length.
impl Deserialize<V50> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = deserialize!(Typecode, V50, ostream, "typecode");
        if typecode::PROPERTIES_OPENNURBS_VERSION == typecode {
            Ok(Begin {
                typecode,
                length: 8u64,
            })
        } else {
            let value = deserialize!(u64, V50, ostream, "value");
            let is_long = (0 == typecode & typecode::SHORT) && (0 < value);
            let length = if is_long { value as u64 } else { 0u64 };
            Ok(Begin { typecode, length })
        }
    }
}

impl Deserialize<V60> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        deserialize!(Begin, V50, ostream)
    }
}

impl Deserialize<V70> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        deserialize!(Begin, V50, ostream)
    }
}

pub struct Chunk<T> {
    pub inner: T,
}

impl<T, V> Deserialize<V> for Chunk<T>
where
    V: FileVersion,
    Begin: Deserialize<V>,
    ErrorStack: From<<Begin as Deserialize<V>>::Error>,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<S>(ostream: &mut S) -> Result<Self, Self::Error>
    where
        S: OStream,
    {
        let begin = deserialize!(Begin, V, ostream, "begin");
        let chunk = &mut ostream.ochunk(Some(begin.length));
        let ret = Self {
            inner: deserialize!(T, V, chunk, "inner"),
        };
        match chunk.seek(SeekFrom::End(0)) {
            Ok(_) => (),
            Err(e) => return Err(ErrorStack::new(Error::IoError(e))),
        }
        Ok(ret)
    }
}

#[derive(Default)]
pub struct BigVersion {
    pub major: u8,
    pub minor: u8,
}

impl<V> Deserialize<V> for BigVersion
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let major = deserialize!(u32, V, ostream, "major");
        let minor = deserialize!(u32, V, ostream, "minor");
        match (
            TryInto::<u8>::try_into(major),
            TryInto::<u8>::try_into(minor),
        ) {
            (Ok(major), Ok(minor)) => Ok(Self { major, minor }),
            _ => Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidChunkVersion,
            ))),
        }
    }
}

impl BigVersion {
    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }
}

#[derive(Default, Deserialize)]
pub struct ShortVersion {
    inner: u8,
}

impl ShortVersion {
    pub fn major(&self) -> u8 {
        self.inner >> 4
    }

    pub fn minor(&self) -> u8 {
        self.inner & 0x0F
    }
}
