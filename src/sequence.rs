use std::marker::PhantomData;

use crate::{
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
};

pub struct Sequence<S, T> {
    pub inner: Vec<T>,
    phantom: PhantomData<S>,
}

impl<V, S, T> Deserialize<V> for Sequence<S, T>
where
    V: FileVersion,
    S: Deserialize<V>,
    ErrorStack: From<<S as Deserialize<V>>::Error>,
    usize: TryFrom<S>,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<U>(stream: &mut once_io::Stream<U>) -> Result<Self, Self::Error>
    where
        U: std::io::Read + std::io::Seek,
    {
        let len = match deserialize!(S, V, stream, "length").try_into() {
            Ok(ok) => ok,
            Err(_) => {
                let mut stack = ErrorStack::new(Error::Simple(ErrorKind::InvalidSequenceLength));
                stack.push_frame("length", std::any::type_name::<S>());
                return Err(stack);
            }
        };
        let mut data: Vec<T> = vec![];
        for _ in 0..len {
            data.push(deserialize!(T, V, stream, "member"));
        }
        Ok(Self {
            inner: data,
            phantom: PhantomData,
        })
    }
}

impl<S, T> Into<Vec<T>> for Sequence<S, T> {
    fn into(self) -> Vec<T> {
        self.inner
    }
}
