use crate::{
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

pub struct U32IntoBool {
    inner: bool,
}

impl<V> Deserialize<V> for U32IntoBool
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(Self {
            inner: <u32 as Deserialize<V>>::deserialize(stream)? != 0,
        })
    }
}

impl Into<bool> for U32IntoBool {
    fn into(self) -> bool {
        self.inner
    }
}
