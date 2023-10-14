use once_3dm_macros::Deserialize;

use crate::{
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
        let typecode = match <Typecode as Deserialize<V1>>::deserialize(ostream) {
            Ok(ok) => ok,
            Err(mut stack) => {
                stack.push_frame("typecode", "Typecode");
                return Err(stack);
            }
        };
        let is_unsigned = 0 == (typecode::SHORT & typecode)
            || typecode::RGB == typecode
            || typecode::RGBDISPLAY == typecode
            || typecode::PROPERTIES_OPENNURBS_VERSION == typecode
            || typecode::OBJECT_RECORD_TYPE == typecode;
        let value = if is_unsigned {
            <u32 as Deserialize<V1>>::deserialize(ostream)? as i64
        } else {
            <i32 as Deserialize<V1>>::deserialize(ostream)? as i64
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
        let typecode = <Typecode as Deserialize<V2>>::deserialize(ostream)?;
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
                <u32 as Deserialize<V2>>::deserialize(ostream)? as i64
            } else {
                <i32 as Deserialize<V2>>::deserialize(ostream)? as i64
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
        <Begin as Deserialize<V2>>::deserialize(ostream)
    }
}

impl Deserialize<V4> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V2>>::deserialize(ostream)
    }
}

//TODO: check is_long and length.
impl Deserialize<V50> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = <Typecode as Deserialize<V50>>::deserialize(ostream)?;
        if typecode::PROPERTIES_OPENNURBS_VERSION == typecode {
            Ok(Begin {
                typecode,
                length: 8u64,
            })
        } else {
            let value = <u64 as Deserialize<V50>>::deserialize(ostream)?;
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
        <Begin as Deserialize<V50>>::deserialize(ostream)
    }
}

impl Deserialize<V70> for Begin {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V50>>::deserialize(ostream)
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
        let major = <u32 as Deserialize<V>>::deserialize(ostream)?;
        let minor = <u32 as Deserialize<V>>::deserialize(ostream)?;
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
