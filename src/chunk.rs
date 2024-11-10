use std::io::{Seek, SeekFrom};

use once_3dm_macros::Deserialize;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::{Error, ErrorKind, ErrorStack},
    type_code::TypeCode,
};

pub struct Begin {
    pub type_code: TypeCode,
    pub length: u64,
}

impl Deserialize<V1> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let type_code = deserialize!(TypeCode, V1, stream, "type_code");
        let is_unsigned = 0 == (TypeCode::Short as u32 & type_code as u32)
            || TypeCode::Rgb == type_code
            || TypeCode::RgbDisplay == type_code
            || TypeCode::PropertiesOpenNurbsVersion == type_code
            || TypeCode::ObjectRecordType == type_code;
        let value = if is_unsigned {
            deserialize!(u32, V1, stream, "length") as i64
        } else {
            deserialize!(i32, V1, stream, "length") as i64
        };
        let is_long = (0 == type_code as u32 & TypeCode::Short as u32)
            && (0 != type_code as u32)
            && (0 < value);
        let length = if is_long { value as u64 } else { 0u64 };
        Ok(Begin { type_code, length })
    }
}

//TODO: check is_long and length.
impl Deserialize<V2> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let type_code = deserialize!(TypeCode, V2, stream, "type_code");
        if TypeCode::PropertiesOpenNurbsVersion == type_code {
            Ok(Begin {
                type_code,
                length: 4u64,
            })
        } else {
            let is_unsigned = 0 == (TypeCode::Short as u32 & type_code as u32)
                || TypeCode::Rgb == type_code
                || TypeCode::RgbDisplay == type_code
                || TypeCode::ObjectRecordType == type_code;
            let value = if is_unsigned {
                deserialize!(u32, V2, stream, "length") as i64
            } else {
                deserialize!(i32, V2, stream, "length") as i64
            };
            let is_long = (0 == type_code as u32 & TypeCode::Short as u32) && (0 < value);
            let length = if is_long { value as u64 } else { 0u64 };
            Ok(Begin { type_code, length })
        }
    }
}

impl Deserialize<V3> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        deserialize!(Begin, V2, stream)
    }
}

impl Deserialize<V4> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        deserialize!(Begin, V2, stream)
    }
}

//TODO: check is_long and length.
impl Deserialize<V50> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let type_code = deserialize!(TypeCode, V50, stream, "type_code");
        if TypeCode::PropertiesOpenNurbsVersion == type_code {
            Ok(Begin {
                type_code,
                length: 8u64,
            })
        } else {
            let value = deserialize!(u64, V50, stream, "value");
            let is_long = (0 == type_code as u32 & TypeCode::Short as u32) && (0 < value);
            let length = if is_long { value as u64 } else { 0u64 };
            Ok(Begin { type_code, length })
        }
    }
}

impl Deserialize<V60> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        deserialize!(Begin, V50, stream)
    }
}

impl Deserialize<V70> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        deserialize!(Begin, V50, stream)
    }
}

pub struct Chunk<const TC: u32, T> {
    pub inner: T,
}

impl<const TC: u32, T, V> Deserialize<V> for Chunk<TC, T>
where
    V: FileVersion,
    Begin: Deserialize<V>,
    ErrorStack: From<<Begin as Deserialize<V>>::Error>,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<S>(stream: &mut once_io::Stream<S>) -> Result<Self, Self::Error>
    where
        S: std::io::Read + std::io::Seek,
    {
        let begin = deserialize!(Begin, V, stream, "begin");
        if TC == TypeCode::Null as u32 || TC == begin.type_code as u32 {
            let chunk = &mut stream.borrow_chunk(Some(begin.length)).unwrap();
            let ret = Self {
                inner: deserialize!(T, V, chunk, "inner"),
            };
            match chunk.seek(SeekFrom::End(0)) {
                Ok(_) => (),
                Err(e) => return Err(ErrorStack::new(Error::IoError(e))),
            }
            Ok(ret)
        } else {
            Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidChunkTypeCode,
            )))
        }
    }
}

#[derive(Default)]
pub struct BigVersion {
    major: u8,
    minor: u8,
}

impl<V> Deserialize<V> for BigVersion
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let major = deserialize!(u32, V, stream, "major");
        let minor = deserialize!(u32, V, stream, "minor");
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
