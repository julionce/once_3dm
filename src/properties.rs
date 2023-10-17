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
    typecode::{self, Typecode},
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

        fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
        where
            T: OStream,
        {
            let mut properties = Properties::default();

            loop {
                let begin = deserialize!(chunk::Begin, V, ostream, "begin");
                let input = &mut ostream.ochunk(Some(begin.length));
                match begin.typecode {
                    typecode::SUMMARY => {
                        properties.revision_history =
                            Some(deserialize!(RevisionHistory, V, input, "revision_history"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    typecode::NOTES => {
                        properties.notes = Some(deserialize!(Notes, V, input, "notes"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    typecode::BITMAPPREVIEW => {
                        properties.preview = Some(deserialize!(Bitmap, V, input, "preview"));
                        input.seek(SeekFrom::End(0)).unwrap();
                    }
                    typecode::CURRENTLAYER | typecode::LAYER => {
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

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v1::Properties as Deserialize<V1>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V2> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v2::Properties as Deserialize<V2>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V3> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v3::Properties as Deserialize<V3>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V4> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v4::Properties as Deserialize<V4>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V50> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v50::Properties as Deserialize<V50>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V60> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v60::Properties as Deserialize<V60>>::deserialize(ostream)?.into())
    }
}

impl Deserialize<V70> for Properties {
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        Ok(<v70::Properties as Deserialize<V70>>::deserialize(ostream)?.into())
    }
}
