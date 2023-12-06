//! STORE instructions store the given value from constant
//!
//! or stack into local variable
//!
//! List:
//!  - ASTORE: Store reference
//!  - DSTORE: Store Double
//!  - FSTORE: Store Float
//!  - ISTORE: Store Integer, Short, Boolean, ...
//!  - LSTORE: Store Long
//!
mod astore;
mod dstore;
mod fstore;
mod istore;
mod lstore;
mod xastore;

pub use self::astore::*;
pub use self::dstore::*;
pub use self::fstore::*;
pub use self::istore::*;
pub use self::lstore::*;
pub use self::xastore::*;
