use crate::error::{Error, ErrorKind, ErrorStack};
use once_io::OStream;
use std::io::Read;

use std::mem;

pub enum V1 {}
pub enum V2 {}
pub enum V3 {}
pub enum V4 {}
pub enum V50 {}
pub enum V60 {}
pub enum V70 {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::V1 {}
    impl Sealed for super::V2 {}
    impl Sealed for super::V3 {}
    impl Sealed for super::V4 {}
    impl Sealed for super::V50 {}
    impl Sealed for super::V60 {}
    impl Sealed for super::V70 {}
}

pub trait FileVersion: private::Sealed {}
impl<T> FileVersion for T where T: private::Sealed {}

pub trait Deserialize<V>
where
    Self: Sized,
    V: FileVersion,
{
    type Error: Into<ErrorStack>;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream;
}

#[macro_export]
macro_rules! deserialize {
    ($type: ty, $version: ty, $ostream: expr) => {
        <$type as Deserialize<$version>>::deserialize($ostream)
    };
    ($type: ty, $version: ty, $ostream: expr, $member: tt) => {
        match <$type as Deserialize<$version>>::deserialize($ostream) {
            Ok(ok) => ok,
            Err(e) => {
                let mut stack: ErrorStack = From::from(e);
                stack.push_frame($member, stringify!($type));
                return Err(stack);
            }
        }
    };
}

impl<V> Deserialize<V> for ()
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(_ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(())
    }
}

macro_rules! impl_deserialize_for_num {
    ($type: ty) => {
        impl<V> Deserialize<V> for $type
        where
            V: FileVersion,
        {
            type Error = ErrorStack;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream,
            {
                let mut buf = [0u8; mem::size_of::<$type>()];
                match ostream.read_exact(&mut buf) {
                    Ok(()) => Ok(<$type>::from_le_bytes(buf)),
                    Err(e) => Err(ErrorStack::new(Error::IoError(e))),
                }
            }
        }
    };
}

impl_deserialize_for_num! {u8}
impl_deserialize_for_num! {u16}
impl_deserialize_for_num! {u32}
impl_deserialize_for_num! {u64}
impl_deserialize_for_num! {u128}
impl_deserialize_for_num! {i8}
impl_deserialize_for_num! {i16}
impl_deserialize_for_num! {i32}
impl_deserialize_for_num! {i64}
impl_deserialize_for_num! {i128}
impl_deserialize_for_num! {usize}
impl_deserialize_for_num! {isize}
impl_deserialize_for_num! {f32}
impl_deserialize_for_num! {f64}

impl<V> Deserialize<V> for bool
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        //TODO: check if values distinct from 0 or 1 are valid.
        let inner = deserialize!(u8, V, ostream)?;
        Ok(0 < inner)
    }
}

impl Deserialize<V1> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let length = deserialize!(u32, V1, ostream, "length");
        let mut string = String::new();
        match ostream.take(length as u64).read_to_string(&mut string) {
            Ok(size) => {
                if size as u64 == length as u64 {
                    Ok(string)
                } else {
                    Err(ErrorStack::new(Error::Simple(
                        ErrorKind::InvalidStringLength,
                    )))
                }
            }
            Err(e) => Err(ErrorStack::new(Error::IoError(e))),
        }
    }
}

impl Deserialize<V2> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let length = <u32 as Deserialize<V2>>::deserialize(ostream)?;
        if 0 < length {
            let mut buf: Vec<u16> = vec![];
            for _ in 0..(length - 1) {
                buf.push(<u16 as Deserialize<V2>>::deserialize(ostream)?);
            }
            <u16 as Deserialize<V2>>::deserialize(ostream)?;
            match String::from_utf16(&buf) {
                Ok(string) => Ok(string),
                Err(e) => Err(ErrorStack::new(Error::FromUtf16Error(e))),
            }
        } else {
            Ok(String::new())
        }
    }
}

impl Deserialize<V3> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        deserialize!(String, V2, ostream)
    }
}

impl Deserialize<V4> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        deserialize!(String, V2, ostream)
    }
}

impl Deserialize<V50> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        deserialize!(String, V2, ostream)
    }
}

impl Deserialize<V60> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        deserialize!(String, V2, ostream)
    }
}

impl Deserialize<V70> for String {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        deserialize!(String, V2, ostream)
    }
}

impl<V, T> Deserialize<V> for Vec<T>
where
    V: FileVersion,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<U>(ostream: &mut U) -> Result<Self, Self::Error>
    where
        U: OStream,
    {
        match ostream.stream_len() {
            Ok(stream_len) => {
                let len = stream_len / mem::size_of::<T>() as u64;
                let mut data: Vec<T> = vec![];
                for _ in 0..len {
                    data.push(T::deserialize(ostream)?);
                }
                Ok(data)
            }
            Err(e) => Err(ErrorStack::new(Error::IoError(e))),
        }
    }
}

impl<V, T, const N: usize> Deserialize<V> for [T; N]
where
    V: FileVersion,
    T: Deserialize<V> + Default + Copy,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<U>(ostream: &mut U) -> Result<Self, Self::Error>
    where
        U: OStream,
    {
        let mut ret = [T::default(); N];
        for elem in ret.iter_mut() {
            *elem = <T as Deserialize<V>>::deserialize(ostream)?;
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    macro_rules! generate_deserialize_num_test {
        ($test_name: ident, $type: ty, $value: expr) => {
            #[test]
            fn $test_name() {
                let data = $value.to_le_bytes();
                let mut reader = Cursor::new(data);
                assert_eq!(deserialize!($type, V1, &mut reader).unwrap(), $value);
            }
        };
    }

    generate_deserialize_num_test! {deserialize_u8_ram_val, u8, 11u8}
    generate_deserialize_num_test! {deserialize_u8_max_val, u8, u8::MAX}
    generate_deserialize_num_test! {deserialize_u8_min_val, u8, u8::MIN}
    generate_deserialize_num_test! {deserialize_u16_ram_val, u16, 11u16}
    generate_deserialize_num_test! {deserialize_u16_max_val, u16, u16::MAX}
    generate_deserialize_num_test! {deserialize_u16_min_val, u16, u16::MIN}
    generate_deserialize_num_test! {deserialize_u32_ram_val, u32, 11u32}
    generate_deserialize_num_test! {deserialize_u32_max_val, u32, u32::MAX}
    generate_deserialize_num_test! {deserialize_u32_min_val, u32, u32::MIN}
    generate_deserialize_num_test! {deserialize_u64_ram_val, u64, 11u64}
    generate_deserialize_num_test! {deserialize_u64_max_val, u64, u64::MAX}
    generate_deserialize_num_test! {deserialize_u64_min_val, u64, u64::MIN}
    generate_deserialize_num_test! {deserialize_u128_ram_val, u128, 11u128}
    generate_deserialize_num_test! {deserialize_u128_max_val, u128, u128::MAX}
    generate_deserialize_num_test! {deserialize_u128_min_val, u128, u128::MIN}
    generate_deserialize_num_test! {deserialize_i8_ram_val, i8, 11i8}
    generate_deserialize_num_test! {deserialize_i8_max_val, i8, i8::MAX}
    generate_deserialize_num_test! {deserialize_i8_min_val, i8, i8::MIN}
    generate_deserialize_num_test! {deserialize_i16_ram_val, i16, 11i16}
    generate_deserialize_num_test! {deserialize_i16_max_val, i16, i16::MAX}
    generate_deserialize_num_test! {deserialize_i16_min_val, i16, i16::MIN}
    generate_deserialize_num_test! {deserialize_i32_ram_val, i32, 11i32}
    generate_deserialize_num_test! {deserialize_i32_max_val, i32, i32::MAX}
    generate_deserialize_num_test! {deserialize_i32_min_val, i32, i32::MIN}
    generate_deserialize_num_test! {deserialize_i64_ram_val, i64, 11i64}
    generate_deserialize_num_test! {deserialize_i64_max_val, i64, i64::MAX}
    generate_deserialize_num_test! {deserialize_i64_min_val, i64, i64::MIN}
    generate_deserialize_num_test! {deserialize_i128_ram_val, i128, 11i128}
    generate_deserialize_num_test! {deserialize_i128_max_val, i128, i128::MAX}
    generate_deserialize_num_test! {deserialize_i128_min_val, i128, i128::MIN}
    generate_deserialize_num_test! {deserialize_usize_ram_val, usize, 11usize}
    generate_deserialize_num_test! {deserialize_usize_max_val, usize, usize::MAX}
    generate_deserialize_num_test! {deserialize_usize_min_val, usize, usize::MIN}
    generate_deserialize_num_test! {deserialize_isize_ram_val, isize, 11isize}
    generate_deserialize_num_test! {deserialize_isize_max_val, isize, isize::MAX}
    generate_deserialize_num_test! {deserialize_isize_min_val, isize, isize::MIN}
    generate_deserialize_num_test! {deserialize_f32_ram_val, f32, 11f32}
    generate_deserialize_num_test! {deserialize_f32_max_val, f32, f32::MAX}
    generate_deserialize_num_test! {deserialize_f32_min_val, f32, f32::MIN}
    generate_deserialize_num_test! {deserialize_f64_ram_val, f64, 11f64}
    generate_deserialize_num_test! {deserialize_f64_max_val, f64, f64::MAX}
    generate_deserialize_num_test! {deserialize_f64_min_val, f64, f64::MIN}

    #[test]
    fn deserialize_vector_size_fill() {
        let data = [11u8, 0, 89u8, 0];
        let mut reader = Cursor::new(data);
        let result = deserialize!(Vec<u16>, V1, &mut reader).unwrap();
        assert_eq!(result, vec![11u16, 89u16]);
    }

    #[test]
    fn deserialize_vector_size_not_fill() {
        let data = [11u8, 0, 89u8, 0, 0];
        let mut reader = Cursor::new(data);
        let result = deserialize!(Vec<u16>, V1, &mut reader).unwrap();
        assert_eq!(result, vec![11u16, 89u16]);
    }
}
