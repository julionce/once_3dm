use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    mesh_parameters::MeshParameters,
    plugin::PluginList,
    rollback::Rollback,
    typecode::{self, Typecode},
    units_and_tolerances::UnitsAndTolerances,
};

#[derive(Default, Deserialize)]
#[table]
pub struct Settings {
    #[field(SETTINGS_PLUGINLIST)]
    pub plugin_list: PluginList,
    #[field(SETTINGS_MODEL_URL)]
    pub model_url: String,
    #[field(SETTINGS_RENDERMESH)]
    pub render_mesh: MeshParameters,
    #[field(SETTINGS_UNITSANDTOLS)]
    pub units_and_tolerances: UnitsAndTolerances,
}
