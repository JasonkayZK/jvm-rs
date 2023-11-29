use std::path::Path;
use std::{env, fmt, fs};

use crate::classpath::entry::{new_entry, Entry};
use crate::classpath::errors::EntryError;
use crate::classpath::wildcard_entry::WildcardEntry;

const JAVA_HOME: &str = "JAVA_HOME";

pub struct ClasspathImpl {
    boot_classpath: Box<dyn Entry>,
    ext_classpath: Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl ClasspathImpl {
    pub fn parse(jre_option: &str, cp_option: &str) -> Self {
        let boot_classpath = ClasspathImpl::parse_boot_classpath(jre_option);
        let ext_classpath = ClasspathImpl::parse_ext_classpath(jre_option);
        let user_classpath = ClasspathImpl::parse_user_classpath(cp_option);
        ClasspathImpl {
            boot_classpath,
            ext_classpath,
            user_classpath,
        }
    }

    fn parse_boot_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = ClasspathImpl::get_jre_dir(jre_option);
        // jre/lib/*
        let path = Path::new(&jre_dir).join("lib").join("*");
        let jre_lib_path = path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_lib_path))
    }

    fn parse_ext_classpath(jre_option: &str) -> Box<dyn Entry> {
        let jre_dir = ClasspathImpl::get_jre_dir(jre_option);
        // jre/lib/ext/*
        let path = Path::new(&jre_dir).join("lib").join("ext").join("*");
        // 检查目录是否存在/是否有权限
        if fs::metadata(&path).is_err() {
            return Box::<WildcardEntry>::default();
        }

        let jre_ext_path = path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_ext_path))
    }

    fn parse_user_classpath(cp_option: &str) -> Box<dyn Entry> {
        let mut cp = cp_option;
        if cp.is_empty() {
            cp = ".";
        }
        new_entry(cp)
    }

    fn get_jre_dir(jre_option: &str) -> String {
        // 使用用户输入的 -Xjre 选项作为 jre 目录
        if !jre_option.is_empty() {
            let jre_dir = Path::new(jre_option);
            if jre_dir.exists() {
                return jre_option.to_string();
            }
        }

        // 使用当前目录下的 jre 目录
        let jre_dir = Path::new("./jre");
        if jre_dir.exists() {
            return "./jre".to_string();
        }

        // 使用 JAVA_HOME 环境变量
        match env::var(JAVA_HOME) {
            Ok(jh) => {
                if !jh.is_empty() {
                    return Path::new(&jh).join("jre").to_str().unwrap().to_string();
                }
            }
            Err(err) => {
                println!("Get environment JAVA_HOME err: {}", err);
            }
        }
        panic!("{}", "JRE folder not found!")
    }
}

impl Entry for ClasspathImpl {
    /// Read class: Bootstrap => Ext => User
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, EntryError> {
        let class = class_name.to_string() + ".class";
        if let Ok(data) = self.boot_classpath.read_class(&class) {
            Ok(data)
        } else if let Ok(data) = self.ext_classpath.read_class(&class) {
            Ok(data)
        } else {
            match self.user_classpath.read_class(&class) {
                Ok(data) => Ok(data),
                Err(err) => Err(err),
            }
        }
    }
}

impl fmt::Display for ClasspathImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_classpath)
    }
}
