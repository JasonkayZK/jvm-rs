use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeDataAreaError {
    #[error("Wrong runtime data type: expect {0:?}, got {1:?}")]
    WrongVarRefType(String, String),

    #[error("java.lang.StackOverflowError")]
    StackOverflow,

    #[error("jvm stack is empty!")]
    StackEmpty,
}
