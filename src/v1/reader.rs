use std::io::{Read, Result, Seek};

use once_io::{
    read_num::{LittleEndianReader, ReadNum},
    OStream,
};

pub struct Reader<T>
where
    T: OStream,
{
    inner: T,
}

impl<T> Reader<T>
where
    T: OStream,
{
    pub fn new(inner: T) -> Reader<T> {
        Reader { inner }
    }
}

impl<T> Read for Reader<T>
where
    T: OStream,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }
}

impl<T> Seek for Reader<T>
where
    T: OStream,
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        self.inner.seek(pos)
    }
}

impl<T> ReadNum for Reader<T>
where
    T: OStream,
{
    type Reader = LittleEndianReader;
}
