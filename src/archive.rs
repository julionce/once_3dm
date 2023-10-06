use crate::{
    chunk,
    deserialize::{Deserialize, FileVersion, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    header::Header,
    properties::Properties,
    typecode,
    version::Version,
};
use once_3dm_macros::Deserialize;
use once_io::OStream;
use std::io::{Seek, SeekFrom};

#[derive(Default, Deserialize)]
#[table]
pub struct Body {
    #[field(PROPERTIES_TABLE)]
    pub properties: Properties,
}

pub struct Archive {
    pub header: Header,
    pub body: Body,
}

impl Archive {
    pub fn deserialize<T>(ostream: &mut T) -> Result<Self, ErrorStack>
    where
        T: OStream,
    {
        let header = Header::deserialize(ostream)?;
        let body = match header.version {
            Version::V1 => <Body as Deserialize<V1>>::deserialize(ostream)?,
            Version::V2 => <Body as Deserialize<V2>>::deserialize(ostream)?,
            Version::V3 => <Body as Deserialize<V3>>::deserialize(ostream)?,
            Version::V4 => <Body as Deserialize<V4>>::deserialize(ostream)?,
            Version::V50 => <Body as Deserialize<V50>>::deserialize(ostream)?,
            Version::V60 => <Body as Deserialize<V60>>::deserialize(ostream)?,
            Version::V70 => <Body as Deserialize<V70>>::deserialize(ostream)?,
        };
        Ok(Self { header, body })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, File};

    use super::*;

    #[test]
    fn deserialize_v1_ok() {
        let paths = read_dir("resource/v1").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v2_ok() {
        let paths = read_dir("resource/v2").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v3_ok() {
        let paths = read_dir("resource/v3").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v4_ok() {
        let paths = read_dir("resource/v4").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v50_ok() {
        let paths = read_dir("resource/v50").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v60_ok() {
        let paths = read_dir("resource/v60").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    #[test]
    fn deserialize_v70_ok() {
        let paths = read_dir("resource/v60").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut ostream =
                            File::open(dir_entry.path().display().to_string()).unwrap();
                        let archive = Archive::deserialize(&mut ostream);
                        match archive {
                            Ok(_) => assert!(true),
                            Err(e) => assert!(false, "{}", e),
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
