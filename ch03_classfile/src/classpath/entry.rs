use crate::classpath::composite_entry::CompositeEntry;
use std::fmt::Display;
use std::path::Path;

use crate::classpath::dir_entry::DirEntry;
use crate::classpath::errors::EntryError;
use crate::classpath::wildcard_entry::WildcardEntry;
use crate::classpath::zip_entry::ZipEntry;

#[cfg(windows)]
pub const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
pub const PATH_SEPARATOR: char = ':';

pub trait Entry: Display {
    fn read_class(&mut self, classname: &str) -> Result<Vec<u8>, EntryError>;
}

/// 获取指定路径的绝对路径
pub fn absolute(path: &str) -> String {
    let path = Path::new(&path);
    match path.canonicalize() {
        Ok(p) => p.to_str().unwrap().to_string(),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

/// 根据传入的 path 创建对应的 Entry
/// -classpath aaa1/bbb1;aaa2/bbb2 => CompositeEntry
/// -classpath aaa/*               => WildcardEntry
/// -classpath aaa.jar             => ZipEntry
/// -classpath aaa                 => DirEntry
pub fn new_entry(path: &str) -> Box<dyn Entry> {
    if path.contains(PATH_SEPARATOR) {
        return Box::new(CompositeEntry::new(path));
    }
    if path.ends_with('*') {
        return Box::new(WildcardEntry::new(path));
    }
    if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        return Box::new(ZipEntry::new(path));
    }
    Box::new(DirEntry::new(path))
}
