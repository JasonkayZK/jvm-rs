use log::warn;
use std::fmt;

use crate::classpath::entry::{new_entry, Entry, PATH_SEPARATOR};
use crate::classpath::errors::EntryError;

/// 由多个 Entry 组成
pub struct CompositeEntry {
    entries: Vec<Box<dyn Entry>>,
}

impl CompositeEntry {
    pub fn new(path_list: &str) -> Self {
        let path_list = path_list.split(PATH_SEPARATOR);
        let mut entries = vec![];
        for path in path_list {
            entries.push(new_entry(path))
        }
        CompositeEntry { entries }
    }
}

impl Entry for CompositeEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, EntryError> {
        for entry in &mut self.entries {
            match entry.read_class(class_name) {
                Ok(data) => {
                    return Ok(data);
                }
                Err(err) => {
                    // warn!("Warning: reading class err: {}", err);
                }
            }
        }
        Err(EntryError::NotFound(class_name.to_string()))
    }
}

impl fmt::Display for CompositeEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_SEPARATOR.to_string()))
    }
}
