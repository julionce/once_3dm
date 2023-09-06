use crate::deserializer::Deserializer;

pub trait Deserialize<D>
where
    Self: Sized,
    D: Deserializer,
{
    type Error;

    fn deserialize(_: &mut D) -> Result<Self, Self::Error>;
}

macro_rules! impl_deserialize_for_num {
    ($type: ty, $method: ident) => {
        impl<D> Deserialize<D> for $type
        where
            D: Deserializer,
        {
            type Error = String;

            fn deserialize(deserializer: &mut D) -> Result<Self, Self::Error> {
                match deserializer.$method() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    };
}

impl_deserialize_for_num! {u8, read_u8}
impl_deserialize_for_num! {u16, read_u16}
impl_deserialize_for_num! {u32, read_u32}
impl_deserialize_for_num! {u64, read_u64}
impl_deserialize_for_num! {u128, read_u128}
impl_deserialize_for_num! {i8, read_i8}
impl_deserialize_for_num! {i16, read_i16}
impl_deserialize_for_num! {i32, read_i32}
impl_deserialize_for_num! {i64, read_i64}
impl_deserialize_for_num! {i128, read_i128}
impl_deserialize_for_num! {usize, read_usize}
impl_deserialize_for_num! {isize, read_isize}
impl_deserialize_for_num! {f32, read_f32}
impl_deserialize_for_num! {f64, read_f64}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::v1;

    use super::*;

    macro_rules! generate_deserialize_num_test {
        ($test_name: ident, $type: ty, $value: expr) => {
            #[test]
            fn $test_name() {
                let data = $value.to_le_bytes();
                let mut reader = v1::reader::Reader::new(Cursor::new(data));
                assert_eq!(<$type>::deserialize(&mut reader).unwrap(), $value);
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
