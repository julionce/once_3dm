use crate::{
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
};

#[derive(Default)]
pub struct Table {}

impl<V> Deserialize<V> for Table
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(Self {})
    }
}
