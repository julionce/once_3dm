use crate::{
    application::Application,
    bitmap::{Bitmap, CompressedBitmap},
    chunk,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    notes::Notes,
    on_version::OnVersion,
    revision_history::RevisionHistory,
    typecode,
};
use once_3dm_macros::Deserialize;
use once_io::OStream;
use std::io::{Seek, SeekFrom};

#[derive(Default)]
pub struct Properties {
    pub filename: String,
    pub notes: Notes,
    pub revision_history: RevisionHistory,
    pub application: Application,
    pub preview_image: Bitmap,
    pub on_version: OnVersion,
}

mod v1 {
    use super::*;

    #[derive(Default, Deserialize)]
    #[table]
    pub struct Properties {
        #[field(NOTES)]
        pub notes: Notes,
    }
}

mod v2 {

    use super::*;

    #[derive(Default, Deserialize)]
    #[table]
    pub struct Properties {
        #[field(PROPERTIES_AS_FILE_NAME)]
        pub filename: String,
        #[field(PROPERTIES_NOTES)]
        pub notes: Notes,
        #[field(PROPERTIES_REVISIONHISTORY)]
        pub revision_history: RevisionHistory,
        #[field(PROPERTIES_APPLICATION)]
        pub application: Application,
        #[field(PROPERTIES_PREVIEWIMAGE)]
        pub preview_image: Bitmap,
        #[field(PROPERTIES_COMPRESSED_PREVIEWIMAGE)]
        #[underlying_type(CompressedBitmap)]
        pub compresed_preview_image: Bitmap,
        #[field(PROPERTIES_OPENNURBS_VERSION)]
        pub on_version: OnVersion,
    }
}

impl From<v1::Properties> for Properties {
    fn from(_value: v1::Properties) -> Self {
        Self::default()
    }
}

impl From<v2::Properties> for Properties {
    fn from(value: v2::Properties) -> Self {
        Self {
            filename: value.filename,
            notes: value.notes,
            revision_history: value.revision_history,
            application: value.application,
            preview_image: value.compresed_preview_image,
            on_version: value.on_version,
        }
    }
}

impl Deserialize<V1> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v1::Properties as Deserialize<V1>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V2> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V3> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V4> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V50> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V60> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V70> for Properties {
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}
