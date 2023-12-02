//! Operand STACK related instructions implementation
//!
//! List:
//!  - POP: pop values on top of the operand stack
//!  - DUP: duplicate values on top of the operand stack
//!  - SWAP: swap values on top of the operand stack
//!
pub use self::dup::*;
pub use self::pop::*;
pub use self::swap::*;

mod dup;
mod pop;
mod swap;
