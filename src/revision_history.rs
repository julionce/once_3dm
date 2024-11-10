use crate::{
    chunk, deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    time::Time,
};
use once_3dm_macros::Deserialize;

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
    #[with_chunk_version(short)]
    #[if_major_version(Eq(1))]
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
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v1::RevisionHistory as Deserialize<V1>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V2> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v2::RevisionHistory as Deserialize<V2>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V3> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v3::RevisionHistory as Deserialize<V3>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V4> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v4::RevisionHistory as Deserialize<V4>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V50> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v50::RevisionHistory as Deserialize<V50>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V60> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v60::RevisionHistory as Deserialize<V60>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V70> for RevisionHistory {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v70::RevisionHistory as Deserialize<V70>>::deserialize(stream)?.into())
    }
}
