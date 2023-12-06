use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeHeapError {
    #[error("java.lang.IllegalAccessError")]
    IllegalAccess,

    #[error("No constants at index {0:?}")]
    ConstantNotFound(usize),

    #[error("java.lang.NoSuchFieldError")]
    NoSuchField,

    #[error("java.lang.ClassNotFoundException: {0:?}")]
    ClassNotFound(String),

    #[error("Parse classfile err: {0:?}")]
    ParseClassFailed(String),

    #[error("Unknown Array type: {0:?}")]
    UnknownArrayType(u8),

    #[error("Not array class")]
    NotArrayClass(String),
}
