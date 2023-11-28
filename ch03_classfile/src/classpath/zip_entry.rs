use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip::ZipArchive;

use crate::classpath::entry::{absolute, Entry};
use crate::error::entry::EntryError;

/// ZIP 或 JAR 文件形式的类路径
pub struct ZipEntry {
    abs_path: String,
    zip_archive: ZipArchive<File>,
}

impl ZipEntry {
    pub fn new(path: &str) -> Self {
        let abs_path = absolute(path);
        let path = Path::new(&abs_path);
        let zip_file = match File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open {:?}: {:?}", path, err),
        };

        ZipEntry {
            abs_path,
            zip_archive: ZipArchive::new(zip_file).unwrap(),
        }
    }
}

impl Entry for ZipEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, EntryError> {
        let archive = &mut self.zip_archive;
        let mut file = match archive.by_name(class_name) {
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

impl Display for ZipEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}
