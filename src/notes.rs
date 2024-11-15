use once_3dm_macros::Deserialize;

use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
};

#[derive(Default)]
pub struct Notes {
    pub visible: i32,
    pub window_left: i32,
    pub window_right: i32,
    pub window_bottom: i32,
    pub data: String,
    pub html_encoded: Option<i32>,
}

mod v1 {
    use super::*;

    #[derive(Default, Deserialize)]
    pub struct Notes {
        pub visible: i32,
        pub window_left: i32,
        pub window_top: i32,
        pub window_right: i32,
        pub window_bottom: i32,
        pub data: String,
    }
}

mod v2 {
    use super::*;

    #[derive(Default, Deserialize)]
    #[with_chunk_version(short)]
    #[if_major_version(Eq(1))]
    pub struct Notes {
        pub html_encoded: i32,
        pub data: String,
        pub visible: i32,
        pub window_left: i32,
        pub window_right: i32,
        pub window_bottom: i32,
    }
}

mod v3 {
    use super::v2;

    pub type Notes = v2::Notes;
}

mod v4 {
    use super::v2;

    pub type Notes = v2::Notes;
}

mod v50 {
    use super::v2;

    pub type Notes = v2::Notes;
}

mod v60 {
    use super::v2;

    pub type Notes = v2::Notes;
}

mod v70 {
    use super::v2;

    pub type Notes = v2::Notes;
}

impl From<v1::Notes> for Notes {
    fn from(value: v1::Notes) -> Self {
        Self {
            visible: value.visible,
            window_left: value.window_left,
            window_right: value.window_right,
            window_bottom: value.window_bottom,
            data: value.data,
            html_encoded: None,
        }
    }
}

impl From<v2::Notes> for Notes {
    fn from(value: v2::Notes) -> Self {
        Self {
            visible: value.visible,
            window_left: value.window_left,
            window_right: value.window_right,
            window_bottom: value.window_bottom,
            data: value.data,
            html_encoded: Some(value.html_encoded),
        }
    }
}

impl Deserialize<V1> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v1::Notes as Deserialize<V1>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V2> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v2::Notes as Deserialize<V2>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V3> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v3::Notes as Deserialize<V3>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V4> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v4::Notes as Deserialize<V4>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V50> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v50::Notes as Deserialize<V50>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V60> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v60::Notes as Deserialize<V60>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V70> for Notes {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v70::Notes as Deserialize<V70>>::deserialize(stream)?.into())
    }
}
