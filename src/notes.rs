use once_3dm_macros::Deserialize;

use once_io::OStream;

use crate::{
    chunk,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
};

#[derive(Default)]
pub struct Notes {
    pub visible: i32,
    pub window_left: i32,
    pub window_right: i32,
    pub window_botton: i32,
    pub data: String,
    pub html_encoded: Option<i32>,
}

mod v1 {
    use super::*;

    #[derive(Default, Deserialize)]
    pub struct Notes {
        pub visible: i32,
        pub window_left: i32,
        pub window_right: i32,
        pub window_botton: i32,
        pub data: String,
    }
}

mod v2 {
    use super::*;

    #[derive(Default, Deserialize)]
    #[chunk_version(short)]
    #[from_chunk_version((1, 0))]
    pub struct Notes {
        pub html_encoded: i32,
        pub data: String,
        pub visible: i32,
        pub window_left: i32,
        pub window_right: i32,
        pub window_botton: i32,
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
            window_botton: value.window_botton,
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
            window_botton: value.window_botton,
            data: value.data,
            html_encoded: Some(value.html_encoded),
        }
    }
}

impl Deserialize<V1> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v1::Notes as Deserialize<V1>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V2> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Notes as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V3> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v3::Notes as Deserialize<V3>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V4> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v4::Notes as Deserialize<V4>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V50> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v50::Notes as Deserialize<V50>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V60> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v60::Notes as Deserialize<V60>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V70> for Notes {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v70::Notes as Deserialize<V70>>::deserialize(ostream)?.into())
    }
}
