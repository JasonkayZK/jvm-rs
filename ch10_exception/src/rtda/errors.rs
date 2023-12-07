use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeDataAreaError {
    #[error("Wrong runtime data type: expect {0:?}, got {1:?}")]
    WrongVarRefType(String, String),

    #[error("java.lang.StackOverflowError")]
    StackOverflow,

    #[error("jvm stack is empty!")]
    StackEmpty,

    #[error("java.lang.IncompatibleClassChangeError")]
    IncompatibleClassChange,

    #[error("java.lang.NoSuchMethodError")]
    NoSuchMethod,

    #[error("java.lang.IllegalAccessError")]
    IllegalAccessError,
}
