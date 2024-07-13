use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    sequence::Sequence,
    type_code::TypeCode,
    uuid::Uuid,
};

#[derive(Default, Deserialize)]
pub struct PluginV1_0 {
    pub id: Uuid,
    pub kind: u32,
    pub name: String,
    pub filename: String,
}

#[derive(Default, Deserialize)]
pub struct PluginV1_1 {
    pub id: Uuid,
    pub kind: u32,
    pub name: String,
    pub filename: String,
    pub organization: String,
    pub address: String,
    pub country: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub update_url: String,
    pub fax: String,
}

#[derive(Default, Deserialize)]
pub struct PluginV1_2 {
    pub id: Uuid,
    pub kind: u32,
    pub name: String,
    pub filename: String,
    pub organization: String,
    pub address: String,
    pub country: String,
    pub phone: String,
    pub email: String,
    pub website: String,
    pub update_url: String,
    pub fax: String,
    pub platform: u32,
    pub sdk_version: u32,
    pub sdk_service_release: u32,
}

#[derive(Default, Deserialize)]
#[with_chunk_version(big)]
pub enum Plugin {
    #[default]
    Empty,
    #[if_chunk_version((1, 0))]
    V1_0(PluginV1_0),
    #[if_chunk_version((1, 1))]
    V1_1(PluginV1_1),
    #[if_chunk_version((1, 2))]
    V1_2(PluginV1_2),
}

#[derive(Default, Deserialize)]
pub struct PluginRef {
    #[in_chunk(AnonymousChunk)]
    pub plugin: Plugin,
}

#[derive(Default, Deserialize)]
#[with_chunk_version(short)]
#[if_major_version(Eq(1))]
pub struct PluginList {
    #[underlying_type(Sequence<u32, PluginRef>)]
    pub plugins: Vec<PluginRef>,
}
