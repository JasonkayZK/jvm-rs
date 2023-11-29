use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("Read class failed: {0:?}")]
    ReadClass(String),

    #[error("Class {0:?} not found")]
    NotFound(String),
}
