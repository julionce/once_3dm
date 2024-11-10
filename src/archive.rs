use crate::{
    body::Body,
    deserialize::{Deserialize, V1, V2, V3, V4, V50, V60, V70},
    error::ErrorStack,
    header::Header,
    version::Version,
};

pub struct Archive {
    pub header: Header,
    pub body: Body,
}

impl Archive {
    pub fn deserialize<T>(stream: &mut once_io::Stream<T>) -> Result<Self, ErrorStack>
    where
        T: std::io::Read + std::io::Seek,
    {
        let header = Header::deserialize(stream)?;
        let body = match match header.version {
            Version::V1 => <Body as Deserialize<V1>>::deserialize(stream),
            Version::V2 => <Body as Deserialize<V2>>::deserialize(stream),
            Version::V3 => <Body as Deserialize<V3>>::deserialize(stream),
            Version::V4 => <Body as Deserialize<V4>>::deserialize(stream),
            Version::V50 => <Body as Deserialize<V50>>::deserialize(stream),
            Version::V60 => <Body as Deserialize<V60>>::deserialize(stream),
            Version::V70 => <Body as Deserialize<V70>>::deserialize(stream),
        } {
            Ok(ok) => ok,
            Err(e) => {
                let mut stack = ErrorStack::from(e);
                stack.push_frame("body", "Body");
                return Err(stack);
            }
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
        let paths = read_dir("resource/v70").unwrap();
        for path in paths {
            let dir_entry = path.as_ref().unwrap();
            match path.as_ref().unwrap().path().extension() {
                Some(extension) => {
                    if extension == "3dm" {
                        let mut file = File::open(dir_entry.path().display().to_string()).unwrap();
                        let mut stream = once_io::Stream::new(&mut file);
                        let archive = Archive::deserialize(&mut stream);
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
