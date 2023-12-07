use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstructionError {
    #[error("java.lang.ArithmeticException: / by zero")]
    ArithmeticDividedByZero,

    #[error("java.lang.ClassCastException")]
    ClassCastException,

    #[error("java.lang.IncompatibleClassChangeError")]
    IncompatibleClassChangeError,

    #[error("java.lang.NullPointerException")]
    NullPointerException,

    #[error("java.lang.IllegalAccessError")]
    IllegalAccessError,

    #[error("java.lang.AbstractMethodError")]
    AbstractMethodError,

    #[error("java.lang.NoSuchMethodError")]
    NoSuchMethodError,

    #[error("java.lang.NegativeArraySizeException: {0:?}")]
    NegativeArraySizeException(i32),

    #[error("java.lang.ArrayIndexOutOfBoundsException: length: {0:?}, index: {1:?}")]
    ArrayIndexOutOfBoundsException(usize, i32),

    #[error("java.lang.UnsatisfiedLinkError: {0:?}")]
    UnsatisfiedLinkError(String),
}
