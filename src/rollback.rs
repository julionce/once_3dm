use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorStack},
};

use std::io::SeekFrom;

pub struct Rollback<T> {
    pub inner: T,
}

impl<T, V> Deserialize<V> for Rollback<T>
where
    V: FileVersion,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<S>(ostream: &mut S) -> Result<Self, Self::Error>
    where
        S: once_io::OStream,
    {
        let rollback_position = match ostream.stream_position() {
            Ok(ok) => ok,
            Err(e) => {
                return Err(ErrorStack::new(Error::IoError(e)));
            }
        };
        let ret = Self {
            inner: deserialize!(T, V, ostream, "inner"),
        };
        match ostream.seek(SeekFrom::Start(rollback_position)) {
            Ok(_) => (),
            Err(e) => {
                return Err(ErrorStack::new(Error::IoError(e)));
            }
        };
        Ok(ret)
    }
}
