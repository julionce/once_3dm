use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    rollback::Rollback,
    typecode::{self, Typecode},
};

#[derive(Default, Deserialize)]
#[table]
pub struct Settings {
    #[field(SETTINGS_MODEL_URL)]
    pub model_url: String,
}
