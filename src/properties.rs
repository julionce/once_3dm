use crate::{
    application::Application,
    bitmap::{Bitmap, CompressedBitmap},
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    notes::Notes,
    on_version::OnVersion,
    revision_history::RevisionHistory,
    rollback::Rollback,
    type_code::TypeCode,
};
use once_3dm_macros::Deserialize;
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

    #[derive(Default)]
    pub struct Properties {
        pub revision_history: Option<RevisionHistory>,
        pub notes: Option<Notes>,
        pub preview: Option<Bitmap>,
    }

    impl<V> Deserialize<V> for Properties
    where
        V: FileVersion,
        chunk::Begin: Deserialize<V>,
        ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
        RevisionHistory: Deserialize<V>,
        ErrorStack: From<<RevisionHistory as Deserialize<V>>::Error>,
        Notes: Deserialize<V>,
        ErrorStack: From<<Notes as Deserialize<V>>::Error>,
        Bitmap: Deserialize<V>,
        ErrorStack: From<<Bitmap as Deserialize<V>>::Error>,
    {
        type Error = ErrorStack;

        fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
        where
            T: std::io::Read + std::io::Seek,
        {
            let mut properties = Properties::default();

            loop {
                let begin = deserialize!(chunk::Begin, V, stream, "begin");
                let input = &mut stream.borrow_chunk(Some(begin.length)).unwrap();
                match begin.type_code {
                    TypeCode::Summary => {
                        properties.revision_history =
                            Some(deserialize!(RevisionHistory, V, input, "revision_history"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    TypeCode::Notes => {
                        properties.notes = Some(deserialize!(Notes, V, input, "notes"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    TypeCode::BitmapPreview => {
                        properties.preview = Some(deserialize!(Bitmap, V, input, "preview"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    TypeCode::CurrentLayer | TypeCode::Layer => {
                        input.seek(SeekFrom::End(0)).unwrap();
                        break;
                    }
                    _ => {
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                }
                if properties.notes.is_some()
                    && properties.revision_history.is_some()
                    && properties.preview.is_none()
                {
                    break;
                }
            }
            Ok(properties)
        }
    }
}

mod v2 {

    use super::*;

    #[derive(Default, Deserialize)]
    #[table]
    pub struct Properties {
        #[field(PropertiesAsFileName)]
        pub filename: String,
        #[field(PropertiesNotes)]
        pub notes: Notes,
        #[field(PropertiesRevisionHistory)]
        pub revision_history: RevisionHistory,
        #[field(PropertiesApplication)]
        pub application: Application,
        #[field(PropertiesPreviewImage)]
        pub preview_image: Bitmap,
        #[field(PropertiesCompressedPreviewImage)]
        #[underlying_type(CompressedBitmap)]
        pub compresed_preview_image: Bitmap,
        #[field(PropertiesOpenNurbsVersion)]
        pub on_version: OnVersion,
    }
}

mod v3 {
    use super::v2;

    pub type Properties = v2::Properties;
}

mod v4 {
    use super::v2;

    pub type Properties = v2::Properties;
}

mod v50 {
    use super::v2;

    pub type Properties = v2::Properties;
}

mod v60 {
    use super::v2;

    pub type Properties = v2::Properties;
}

mod v70 {
    use super::v2;

    pub type Properties = v2::Properties;
}

impl From<v1::Properties> for Properties {
    fn from(value: v1::Properties) -> Self {
        let mut properties = Self::default();
        match value.revision_history {
            Some(r) => properties.revision_history = r,
            _ => (),
        };
        match value.notes {
            Some(n) => properties.notes = n,
            _ => (),
        };
        match value.preview {
            Some(p) => properties.preview_image = p,
            _ => (),
        };
        properties
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
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v1::Properties as Deserialize<V1>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V2> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V3> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v3::Properties as Deserialize<V3>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V4> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v4::Properties as Deserialize<V4>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V50> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v50::Properties as Deserialize<V50>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V60> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v60::Properties as Deserialize<V60>>::deserialize(stream)?.into())
    }
}

impl Deserialize<V70> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        Ok(<v70::Properties as Deserialize<V70>>::deserialize(stream)?.into())
    }
}
