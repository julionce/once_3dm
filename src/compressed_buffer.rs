use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    type_code::TypeCode,
};

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

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let size = deserialize!(u32, V, stream, "size") as u64;
        //TODO: handle crc.
        let _crc = deserialize!(u32, V, stream, "crc");
        let mode = deserialize!(u8, V, stream, "mode");
        match mode {
            0u8 => {
                let mut chunk = stream.borrow_chunk(Some(size)).unwrap();
                Ok(Self {
                    mode: CompressionMode::Uncompressed,
                    size,
                    inner: deserialize!(Vec<u8>, V, &mut chunk, "inner"),
                })
            }
            1u8 => {
                let begin = deserialize!(chunk::Begin, V, stream, "begin");
                match begin.type_code {
                    TypeCode::AnonymousChunk => {
                        let limit = begin.length.checked_sub(4);
                        match limit {
                            Some(v) => {
                                let mut chunk = stream.borrow_chunk(Some(v)).unwrap();
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
