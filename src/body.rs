use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    properties::Properties,
    rollback::Rollback,
    settings::Settings,
    type_code::TypeCode,
};

#[derive(Default)]
pub struct Body {
    pub properties: Properties,
    pub settings: Settings,
}

mod v1 {
    use super::*;

    #[derive(Default, Deserialize)]
    pub struct Body {
        pub properties: Properties,
    }
}

mod v2 {
    use crate::object;

    use super::*;

    #[derive(Default, Deserialize)]
    #[table]
    pub struct Body {
        #[field(PropertiesTable)]
        pub properties: Properties,
        #[field(SettingsTable)]
        pub settings: Settings,
        #[field(ObjectTable)]
        pub object_table: object::Table,
    }
}

mod v3 {
    use super::v2;

    pub type Body = v2::Body;
}

mod v4 {
    use super::v2;

    pub type Body = v2::Body;
}

mod v50 {
    use super::v2;

    pub type Body = v2::Body;
}

mod v60 {
    use super::v2;

    pub type Body = v2::Body;
}

mod v70 {
    use super::v2;

    pub type Body = v2::Body;
}

impl Into<Body> for v1::Body {
    fn into(self) -> Body {
        let mut body = Body::default();
        body.properties = self.properties;
        body
    }
}

impl Into<Body> for v2::Body {
    fn into(self) -> Body {
        Body {
            properties: self.properties,
            settings: self.settings,
        }
    }
}

impl Deserialize<V1> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v1::Body as Deserialize<V1>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V2> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v2::Body as Deserialize<V2>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V3> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v3::Body as Deserialize<V3>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V4> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v4::Body as Deserialize<V4>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V50> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v50::Body as Deserialize<V50>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V60> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v60::Body as Deserialize<V60>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V70> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v70::Body as Deserialize<V70>>::deserialize(stream)?.into())
    }
}
