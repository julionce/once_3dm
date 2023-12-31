use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    type_code::TypeCode,
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
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let size = deserialize!(u32, V, ostream, "size") as u64;
        //TODO: handle crc.
        let _crc = deserialize!(u32, V, ostream, "crc");
        let mode = deserialize!(u8, V, ostream, "mode");
        match mode {
            0u8 => {
                let mut chunk = ostream.ochunk(Some(size));
                Ok(Self {
                    mode: CompressionMode::Uncompressed,
                    size,
                    inner: deserialize!(Vec<u8>, V, &mut chunk, "inner"),
                })
            }
            1u8 => {
                let begin = deserialize!(chunk::Begin, V, ostream, "begin");
                match begin.type_code {
                    TypeCode::AnonymousChunk => {
                        let limit = begin.length.checked_sub(4);
                        match limit {
                            Some(v) => {
                                let mut chunk = ostream.ochunk(Some(v));
                                //TODO: uncompress buffer using zlib.
                                Ok(Self {
                                    mode: CompressionMode::Compressed,
                                    size,
                                    inner: deserialize!(Vec<u8>, V, &mut chunk, "inner"),
                                })
                            }
                            None => {
                                Err(ErrorStack::new(Error::Simple(ErrorKind::InvalidChunkSize)))
                            }
                        }
                    }
                    _ => Err(ErrorStack::new(Error::Simple(
                        ErrorKind::InvalidChunkTypeCode,
                    ))),
                }
            }
            _ => Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidCompressionMode,
            ))),
        }
    }
}
