use once_io::OStream;

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
    type Error;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream;
}

macro_rules! impl_deserialize_for_num {
    ($type: ty) => {
        impl<V> Deserialize<V> for $type
        where
            V: FileVersion,
        {
            type Error = String;

            fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
            where
                T: OStream,
            {
                let mut buf = [0u8; mem::size_of::<$type>()];
                match ostream.read_exact(&mut buf) {
                    Ok(()) => Ok(<$type>::from_le_bytes(buf)),
                    Err(e) => Err(e.to_string()),
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
                assert_eq!(
                    <$type as Deserialize<V1>>::deserialize(&mut reader).unwrap(),
                    $value
                );
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
}
