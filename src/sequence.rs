use std::marker::PhantomData;

use crate::{
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
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
    ErrorStack: From<<usize as TryFrom<S>>::Error>,
    T: Deserialize<V>,
    ErrorStack: From<<T as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<U>(ostream: &mut U) -> Result<Self, Self::Error>
    where
        U: once_io::OStream,
    {
        let len = S::deserialize(ostream)?.try_into()?;
        let mut data: Vec<T> = vec![];
        for _ in 0..len {
            data.push(T::deserialize(ostream)?);
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
