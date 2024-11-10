use crate::{
    chunk::Begin,
    deserialize,
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
    type_code::TypeCode,
};

use std::io::Read;

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

    fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, Self::Error>
    where
        T: std::io::Read + std::io::Seek,
    {
        let begin = deserialize!(Begin, V, stream, "begin");
        if TypeCode::CommentBlock == begin.type_code {
            let mut chunk = stream.borrow_chunk(Some(begin.length)).unwrap();
            let mut string = String::new();
            match chunk.read_to_string(&mut string) {
                Ok(_) => Ok(Comment(string)),
                Err(e) => Err(ErrorStack::new(Error::IoError(e))),
            }
        } else {
            Err(ErrorStack::new(Error::Simple(
                ErrorKind::InvalidChunkTypeCode,
            )))
        }
    }
}
