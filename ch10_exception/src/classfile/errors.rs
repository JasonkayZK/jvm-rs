use thiserror::Error;

pub type ClassFileResult<T> = Result<T, ClassFileError>;

#[derive(Error, Debug)]
pub enum ClassFileError {
    #[error("java.lang.ClassFormatError, wrong magic: {0:x}")]
    WrongMagicNumber(u32),

    #[error("java.lang.UnsupportedClassVersionError, unsupported version{0:?}")]
    UnsupportedClassVersionError(u16),
}
