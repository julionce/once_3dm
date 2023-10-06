use once_io::OStream;

use crate::{
    deserialize::{Deserialize, FileVersion},
    error::{Error, ErrorKind, ErrorStack},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
    V50,
    V60,
    V70,
}

impl TryFrom<u8> for Version {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Version::V1),
            2 => Ok(Version::V2),
            3 => Ok(Version::V3),
            4 => Ok(Version::V4),
            50 => Ok(Version::V50),
            60 => Ok(Version::V60),
            70 => Ok(Version::V70),
            _ => Err(Error::Simple(ErrorKind::InvalidVersion)),
        }
    }
}

impl Into<u8> for Version {
    fn into(self) -> u8 {
        match self {
            Version::V1 => 1,
            Version::V2 => 2,
            Version::V3 => 3,
            Version::V4 => 4,
            Version::V50 => 50,
            Version::V60 => 60,
            Version::V70 => 70,
        }
    }
}

impl<V> Deserialize<V> for Version
where
    V: FileVersion,
{
    type Error = ErrorStack;

    fn deserialize<T>(ostream: &mut T) -> Result<Self, Self::Error>
    where
        T: OStream,
    {
        let mut buf = [0; 8];
        match ostream.read_exact(&mut buf) {
            Ok(()) => {
                match buf
                    .iter()
                    .skip_while(|x| **x == ' ' as u8)
                    .try_fold(0u8, |acc, x| match (*x as char).to_digit(10) {
                        Some(d) => Ok(acc * 10u8 + (d as u8)),
                        None => Err(Error::Simple(ErrorKind::InvalidVersion)),
                    }) {
                    Ok(v) => match Version::try_from(v) {
                        Ok(version) => Ok(version),
                        Err(e) => Err(ErrorStack::new(e)),
                    },
                    Err(e) => Err(ErrorStack::new(e)),
                }
            }
            Err(e) => Err(ErrorStack::new(Error::IoError(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::deserialize::V1;

    use super::*;

    #[test]
    fn conversions() {
        let mut version = Version::V1;
        assert_eq!(1u8, version.into());
        version = Version::V2;
        assert_eq!(2u8, version.into());
        version = Version::V3;
        assert_eq!(3u8, version.into());
        version = Version::V4;
        assert_eq!(4u8, version.into());
        version = Version::V50;
        assert_eq!(50u8, version.into());
        version = Version::V60;
        assert_eq!(60u8, version.into());
        version = Version::V70;
        assert_eq!(70u8, version.into());

        assert_eq!(Version::try_from(1u8).ok(), Some(Version::V1));
        assert_eq!(Version::try_from(2u8).ok(), Some(Version::V2));
        assert_eq!(Version::try_from(3u8).ok(), Some(Version::V3));
        assert_eq!(Version::try_from(4u8).ok(), Some(Version::V4));
        assert_eq!(Version::try_from(50u8).ok(), Some(Version::V50));
        assert_eq!(Version::try_from(60u8).ok(), Some(Version::V60));
        assert_eq!(Version::try_from(70u8).ok(), Some(Version::V70));
        assert_eq!(
            Version::try_from(0u8).err().unwrap().kind(),
            ErrorKind::InvalidVersion
        );
    }

    #[test]
    fn deserialize_ok() {
        let data = "       1".as_bytes();
        let mut ostream = Cursor::new(data);
        assert_eq!(
            <Version as Deserialize<V1>>::deserialize(&mut ostream).ok(),
            Some(Version::V1)
        );
    }

    #[test]
    fn deserialize_invalid_version() {
        let data = "        a".as_bytes();
        let mut ostream = Cursor::new(data);
        assert!(<Version as Deserialize<V1>>::deserialize(&mut ostream).is_err());
    }

    #[test]
    fn deserialize_io_error() {
        let data = "    1".as_bytes();
        let mut ostream = Cursor::new(data);
        assert!(<Version as Deserialize<V1>>::deserialize(&mut ostream).is_err());
    }
}
