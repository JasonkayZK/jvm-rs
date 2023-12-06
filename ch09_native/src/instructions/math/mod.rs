//! Mathematical instructions implementation
//!
//! List:
//!  - ADD: addition
//!  - SUB: subtraction
//!  - MUL: multiplication
//!  - DIV: division
//!  - REM: remainder
//!  - NEG: negative
//!  - AND: and
//!  - OR: or
//!  - XOR: exclusive or
//!  - IINC: increment
//!  - SH: shift
//!
mod add;
mod and;
mod div;
mod iinc;
mod mul;
mod neg;
mod or;
mod rem;
mod sh;
mod sub;
mod xor;

pub use self::add::*;
pub use self::and::*;
pub use self::div::*;
pub use self::iinc::*;
pub use self::mul::*;
pub use self::neg::*;
pub use self::or::*;
pub use self::rem::*;
pub use self::sh::*;
pub use self::sub::*;
pub use self::xor::*;
