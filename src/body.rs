use once_3dm_macros::Deserialize;

use once_io::OStream;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    properties::Properties,
    rollback::Rollback,
    settings::Settings,
    typecode::{self, Typecode},
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
    use super::*;

    #[derive(Default, Deserialize)]
    #[table]
    pub struct Body {
        #[field(PROPERTIES_TABLE)]
        pub properties: Properties,
        #[field(SETTINGS_TABLE)]
        pub settings: Settings,
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

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v1::Body as Deserialize<V1>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V2> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v2::Body as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V3> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v3::Body as Deserialize<V3>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V4> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v4::Body as Deserialize<V4>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V50> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v50::Body as Deserialize<V50>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V60> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v60::Body as Deserialize<V60>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V70> for Body {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        Ok(<v70::Body as Deserialize<V70>>::deserialize(ostream)?.into())
    }
}
