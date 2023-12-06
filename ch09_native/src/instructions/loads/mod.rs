//! LOAD instruction implementation
//!
//! LOAD instruction loads the local variable from current frame to operand stack
//!
//! List:
//!  - ALOAD: loads object reference
//!  - DLOAD: loads double
//!  - FLOAD: loads float
//!  - ILOAD: loads integer, short, boolean, ...
//!  - LLOAD: loads long
//!

pub use self::aload::*;
pub use self::dload::*;
pub use self::fload::*;
pub use self::iload::*;
pub use self::lload::*;
pub use self::xaload::*;

mod aload;
mod dload;
mod fload;
mod iload;
mod lload;
mod xaload;
