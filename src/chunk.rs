use crate::{
    deserialize::{Deserialize, V1, V2, V3, V4, V50, V60, V70},
    typecode::{self, Typecode},
};

pub struct Begin {
    pub typecode: Typecode,
    pub length: u64,
}

impl Deserialize<V1> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = <Typecode as Deserialize<V1>>::deserialize(ostream)?;
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

impl Deserialize<V2> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = <Typecode as Deserialize<V2>>::deserialize(ostream)?;
        let is_unsigned = 0 == (typecode::SHORT & typecode)
            || typecode::RGB == typecode
            || typecode::RGBDISPLAY == typecode
            || typecode::PROPERTIES_OPENNURBS_VERSION == typecode
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

impl Deserialize<V3> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V2>>::deserialize(ostream)
    }
}

impl Deserialize<V4> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V2>>::deserialize(ostream)
    }
}

impl Deserialize<V50> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let typecode = <Typecode as Deserialize<V50>>::deserialize(ostream)?;
        let value = <i64 as Deserialize<V50>>::deserialize(ostream)?;
        let is_long = (0 == typecode & typecode::SHORT) && (0 < value);
        let length = if is_long { value as u64 } else { 0u64 };
        Ok(Begin { typecode, length })
    }
}

impl Deserialize<V60> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V50>>::deserialize(ostream)
    }
}

impl Deserialize<V70> for Begin {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        <Begin as Deserialize<V50>>::deserialize(ostream)
    }
}