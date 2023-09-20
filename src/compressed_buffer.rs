use crate::{
    chunk,
    deserialize::{Deserialize, FileVersion},
    typecode,
};

use once_io::OStream;

pub enum CompressionMode {
    Uncompressed,
    Compressed,
}

pub struct CompressedBuffer {
    pub mode: CompressionMode,
    pub size: u64,
    pub inner: Vec<u8>,
}

impl<V> Deserialize<V> for CompressedBuffer
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
        let size = <u32 as Deserialize<V>>::deserialize(ostream)? as u64;
        //TODO: handle crc.
        let _crc = <u32 as Deserialize<V>>::deserialize(ostream)?;
        let mode = <u8 as Deserialize<V>>::deserialize(ostream)?;
        match mode {
            0u8 => {
                let mut chunk = ostream.ochunk(Some(size));
                Ok(Self {
                    mode: CompressionMode::Uncompressed,
                    size,
                    inner: <Vec<u8> as Deserialize<V>>::deserialize(&mut chunk)?,
                })
            }
            1u8 => {
                let begin = <chunk::Begin as Deserialize<V>>::deserialize(ostream)?;
                match begin.typecode {
                    typecode::ANONYMOUS_CHUNK => {
                        let limit = begin.length.checked_sub(4);
                        match limit {
                            Some(v) => {
                                let mut chunk = ostream.ochunk(Some(v));
                                //TODO: uncompress buffer using zlib.
                                Ok(Self {
                                    mode: CompressionMode::Compressed,
                                    size,
                                    inner: <Vec<u8> as Deserialize<V>>::deserialize(&mut chunk)?,
                                })
                            }
                            None => Err("invalid chunk size".to_string()),
                        }
                    }
                    _ => Err("invalid chunk typecode".to_string()),
                }
            }
            _ => Err("invalid compression mode".to_string()),
        }
    }
}
