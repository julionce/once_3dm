use std::mem::size_of;

use crate::{
    chunk,
    compressed_buffer::CompressedBuffer,
    deserialize::{Deserialize, FileVersion, V1, V2, V3},
};

use once_io::OStream;

use once_3dm_macros::Deserialize;

#[derive(Default)]
pub struct Bitmap {
    pub header: Header,
    pub palette: Palette,
    pub pixels: Pixels,
}

impl Bitmap {
    fn palette_color_count(&self) -> u32 {
        match self.header.clr_used {
            0 => match self.header.bit_count {
                1 => 2,
                4 => 16,
                8 => 256,
                _ => 0,
            },
            v => v,
        }
    }

    fn bytes_per_row(&self) -> Option<usize> {
        match (self.header.bit_count as usize).checked_mul(self.header.width as usize) {
            Some(v) => v.next_multiple_of(32).checked_div(8),
            None => None,
        }
    }
}

#[derive(Default, Deserialize)]
pub struct Header {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: u32,
    pub y_pels_per_meter: u32,
    pub clr_used: u32,
    pub clr_important: u32,
}

pub type Palette = Vec<Color>;

#[derive(Default, Deserialize)]
pub struct Color {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

pub type Pixels = Vec<u8>;

impl<V> Deserialize<V> for Bitmap
where
    V: FileVersion,
{
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let mut bitmap = Bitmap::default();
        bitmap.header = <Header as Deserialize<V>>::deserialize(ostream)?;
        let palette_limit = bitmap.palette_color_count() as u64 * size_of::<Color>() as u64;
        let image_limit = bitmap.header.size_image as u64;
        let mut palette_and_image_chunk = ostream.ochunk(Some(palette_limit + image_limit));
        let mut palette_chunk = palette_and_image_chunk.ochunk(Some(palette_limit));
        bitmap.palette = <Palette as Deserialize<V>>::deserialize(&mut palette_chunk)?;
        palette_and_image_chunk = palette_chunk.into_inner();
        bitmap.pixels = <Pixels as Deserialize<V>>::deserialize(&mut palette_and_image_chunk)?;
        Ok(bitmap)
    }
}

struct CompressedBitmap {
    inner: Bitmap,
}

impl Into<Bitmap> for CompressedBitmap {
    fn into(self) -> Bitmap {
        self.inner
    }
}

impl<V> Deserialize<V> for CompressedBitmap
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    String: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let mut bitmap = Bitmap::default();
        bitmap.header = <Header as Deserialize<V>>::deserialize(ostream)?;

        let palette_limit = bitmap.palette_color_count() as u64 * size_of::<Color>() as u64;
        let image_limit = bitmap.header.size_image as u64;

        let buffer = <CompressedBuffer as Deserialize<V>>::deserialize(ostream)?;
        if buffer.size == palette_limit + image_limit {
            // TODO: conver Vec<u8> to Palette
            // bitmap.palette = buffer.inner[..palette_limit];
            bitmap.pixels = buffer.inner[(palette_limit as usize)..].to_vec();
            Ok(Self { inner: bitmap })
        } else if buffer.size == palette_limit {
            // TODO: conver Vec<u8> to Palette
            // bitmap.palette = buffer.inner;
            let buffer = <CompressedBuffer as Deserialize<V>>::deserialize(ostream)?;
            if buffer.size == image_limit {
                Ok(Self { inner: bitmap })
            } else {
                Err("buffer size mismatch".to_string())
            }
        } else {
            Err("buffer size mismatch".to_string())
        }
    }
}
