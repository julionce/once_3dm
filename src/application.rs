use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
};
use once_3dm_macros::Deserialize;

#[derive(Default)]
pub struct Application {
    pub name: String,
    pub url: String,
    pub details: String,
}

mod v2 {
    use super::*;

    #[derive(Default, Deserialize)]
    #[with_chunk_version(short)]
    pub struct Application {
        pub name: String,
        pub url: String,
        pub details: String,
    }
}

mod v3 {
    use super::*;

    pub type Application = v2::Application;
}

mod v4 {
    use super::*;

    pub type Application = v2::Application;
}

mod v50 {
    use super::*;

    pub type Application = v2::Application;
}

mod v60 {
    use super::*;

    pub type Application = v2::Application;
}

mod v70 {
    use super::*;

    pub type Application = v2::Application;
}

impl From<v2::Application> for Application {
    fn from(value: v2::Application) -> Self {
        Self {
            name: value.name,
            url: value.url,
            details: value.details,
        }
    }
}

impl Deserialize<V2> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v2::Application as Deserialize<V2>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V3> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v3::Application as Deserialize<V3>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V4> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v4::Application as Deserialize<V4>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V50> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v50::Application as Deserialize<V50>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V60> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v60::Application as Deserialize<V60>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V70> for Application {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v70::Application as Deserialize<V70>>::deserialize(stream)?.into())
    }
}
