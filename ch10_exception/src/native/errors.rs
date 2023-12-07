use thiserror::Error;

#[derive(Error, Debug)]
pub enum NativeMethodError {
    #[error("java.lang.ArrayStoreException")]
    ArrayStoreException,

    #[error("java.lang.IndexOutOfBoundsException")]
    IndexOutOfBoundsException,

    #[error("java.lang.NullPointerException")]
    NullPointerException,

    #[error("java.lang.CloneNotSupportedException")]
    CloneNotSupportedException,
}
