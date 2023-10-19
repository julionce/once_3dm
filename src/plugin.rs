use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::ErrorStack,
    sequence::Sequence,
    typecode,
    uuid::Uuid,
};

#[derive(Default, Deserialize)]
#[with_version(big)]
#[if_major_version(Eq(1))]
pub struct Plugin {
    pub id: Uuid,
    pub kind: u32,
    pub name: String,
    pub filename: String,
    #[if_minor_version(Ge(1))]
    pub organization: String,
    #[if_minor_version(Ge(1))]
    pub address: String,
    #[if_minor_version(Ge(1))]
    pub country: String,
    #[if_minor_version(Ge(1))]
    pub phone: String,
    #[if_minor_version(Ge(1))]
    pub email: String,
    #[if_minor_version(Ge(1))]
    pub website: String,
    #[if_minor_version(Ge(1))]
    pub update_url: String,
    #[if_minor_version(Ge(1))]
    pub fax: String,
    #[if_minor_version(Ge(2))]
    pub platform: u32,
    #[if_minor_version(Ge(2))]
    pub sdk_version: u32,
    #[if_minor_version(Ge(2))]
    pub sdk_service_release: u32,
}

#[derive(Default, Deserialize)]
pub struct PluginRef {
    #[in_chunk(ANONYMOUS_CHUNK)]
    pub plugin: Plugin,
}

#[derive(Default, Deserialize)]
#[with_version(short)]
#[if_major_version(Eq(1))]
pub struct PluginList {
    #[underlying_type(Sequence<u32, PluginRef>)]
    pub plugins: Vec<PluginRef>,
}
