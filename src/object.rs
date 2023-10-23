use once_3dm_macros::Deserialize;

use crate::{
    chunk::{self, Chunk},
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    rollback::Rollback,
    type_code::TypeCode,
};

#[derive(Default, Deserialize)]
pub struct Record {
    #[in_chunk(ObjectRecordType)]
    _empty: (),
}

#[derive(Default)]
pub struct Table {
    pub records: Vec<Record>,
}

impl<V> Deserialize<V> for Table
where
    V: FileVersion,
    chunk::Begin: Deserialize<V>,
    ErrorStack: From<<chunk::Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: once_io::OStream,
    {
        let mut records = vec![];
        loop {
            let type_code = deserialize!(Rollback<TypeCode>, V, ostream, "type_code").inner;
            match type_code {
                TypeCode::ObjectRecord => {
                    records.push(
                        deserialize!(
                            Chunk::<{ TypeCode::ObjectRecord as u32 }, Record>,
                            V,
                            ostream,
                            "record"
                        )
                        .inner,
                    );
                }
                TypeCode::EndOfTable => {
                    break;
                }
                _ => {
                    return Err(ErrorStack::new(Error::Simple(
                        ErrorKind::InvalidChunkTypeCode,
                    )));
                }
            };
        }
        Ok(Self { records })
    }
}
