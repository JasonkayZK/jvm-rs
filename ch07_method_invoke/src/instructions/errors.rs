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
}
