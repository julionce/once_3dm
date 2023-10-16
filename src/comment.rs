use once_io::OStream;
use std::io::Read;

use crate::{
    chunk::Begin,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
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
    Begin: Deserialize<V>,
    ErrorStack: From<<Begin as Deserialize<V>>::Error>,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let begin = deserialize!(Begin, V, ostream, "begin");
        if typecode::COMMENTBLOCK == begin.typecode {
            let mut chunk = ostream.ochunk(Some(begin.length));
            let mut string = String::new();
            match chunk.read_to_string(&mut string) {
                Ok(_) => Ok(Comment(string)),
                Err(e) => Err(ErrorStack::new(Error::IoError(e))),
            }
        } else {
            Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidChunkTypecode,
            )))
        }
    }
}
