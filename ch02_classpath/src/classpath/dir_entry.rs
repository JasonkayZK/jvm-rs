use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::classpath::entry::{absolute, Entry};
use crate::error::entry::EntryError;

/// 目录形式的类路径
pub struct DirEntry {
    abs_dir: String,
}

impl DirEntry {
    pub fn new(path: &str) -> Self {
        Self {
            abs_dir: absolute(path),
        }
    }
}

impl Display for DirEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abs_dir)
    }
}

impl Entry for DirEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, EntryError> {
        let path = Path::new(&self.abs_dir);
        let path = path.join(class_name);
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(EntryError::ReadClass(format!(
                    "Read class {} err: {}",
                    class_name, err
                )));
            }
        };
        let mut vec: Vec<u8> = vec![];
        file.read_to_end(&mut vec)
            .map_err(|err| EntryError::ReadClass(err.to_string()))?;

        Ok(vec)
    }
}
