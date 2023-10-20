use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    mesh_parameters::MeshParameters,
    plugin::PluginList,
    rollback::Rollback,
    type_code::TypeCode,
    units_and_tolerances::UnitsAndTolerances,
};

#[derive(Default, Deserialize)]
#[table]
pub struct Settings {
    #[field(SettingsPluginList)]
    pub plugin_list: PluginList,
    #[field(SettingsModelUrl)]
    pub model_url: String,
    #[field(SettingsRenderMesh)]
    pub render_mesh: MeshParameters,
    #[field(SettingsUnitsAndTolerances)]
    pub units_and_tolerances: UnitsAndTolerances,
}
