use crate::{
    chunk,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    time::Time,
};
use once_3dm_macros::Deserialize;
use once_io::OStream;

#[derive(Default)]
pub struct RevisionHistory {
    pub created_by: String,
    pub create_time: Time,
    pub last_edited_by: String,
    pub last_edit_time: Time,
    pub revision_count: i32,
}

mod v1 {

    use super::*;

    #[derive(Default, Deserialize)]
    pub struct RevisionHistory {
        pub created_by: String,
        pub create_time: Time,
        #[padding(i32)]
        pub last_edited_by: String,
        pub last_edit_time: Time,
        #[padding(i32)]
        pub revision_count: i32,
    }
}

mod v2 {

    use super::*;

    #[derive(Default, Deserialize)]
    #[chunk_version(short)]
    #[from_chunk_version((1, 0))] // TODO: change to on_chunk_version
    pub struct RevisionHistory {
        pub created_by: String,
        pub create_time: Time,
        pub last_edited_by: String,
        pub last_edit_time: Time,
        pub revision_count: i32,
    }
}

mod v3 {
    use super::v2;

    pub type RevisionHistory = v2::RevisionHistory;
}

mod v4 {
    use super::v2;

    pub type RevisionHistory = v2::RevisionHistory;
}

mod v50 {
    use super::v2;

    pub type RevisionHistory = v2::RevisionHistory;
}

mod v60 {
    use super::v2;

    pub type RevisionHistory = v2::RevisionHistory;
}

mod v70 {
    use super::v2;

    pub type RevisionHistory = v2::RevisionHistory;
}

impl From<v1::RevisionHistory> for RevisionHistory {
    fn from(value: v1::RevisionHistory) -> Self {
        Self {
            created_by: value.created_by,
            create_time: value.create_time,
            last_edited_by: value.last_edited_by,
            last_edit_time: value.last_edit_time,
            revision_count: value.revision_count,
        }
    }
}

impl From<v2::RevisionHistory> for RevisionHistory {
    fn from(value: v2::RevisionHistory) -> Self {
        Self {
            created_by: value.created_by,
            create_time: value.create_time,
            last_edited_by: value.last_edited_by,
            last_edit_time: value.last_edit_time,
            revision_count: value.revision_count,
        }
    }
}

impl Deserialize<V1> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v1::RevisionHistory as Deserialize<V1>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V2> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::RevisionHistory as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V3> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v3::RevisionHistory as Deserialize<V3>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V4> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v4::RevisionHistory as Deserialize<V4>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V50> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v50::RevisionHistory as Deserialize<V50>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V60> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v60::RevisionHistory as Deserialize<V60>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V70> for RevisionHistory {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v70::RevisionHistory as Deserialize<V70>>::deserialize(ostream)?.into())
    }
}
