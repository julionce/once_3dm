use once_io::{read_num::ReadNum, OStream};

pub trait Deserializer: OStream + ReadNum {}

impl<T> Deserializer for T where T: OStream + ReadNum {}
