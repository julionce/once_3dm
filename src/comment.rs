use once_io::OStream;
use std::io::Read;

use crate::{
    chunk::Begin,
    deserialize::{Deserialize, FileVersion, V1},
    typecode,
};

pub struct Comment(String);

impl From<Comment> for String {
    fn from(comment: Comment) -> Self {
        comment.0
    }
}

impl<V> Deserialize<V> for Comment
where
    V: FileVersion,
{
    type Error = String;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let begin = <Begin as Deserialize<V1>>::deserialize(ostream)?;
        if typecode::COMMENTBLOCK == begin.typecode {
            let mut chunk = ostream.ochunk(Some(begin.length));
            let mut string = String::new();
            match chunk.read_to_string(&mut string) {
                Ok(_) => Ok(Comment(string)),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("invalid typecode".to_string())
        }
    }
}
