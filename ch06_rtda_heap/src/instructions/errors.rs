use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstructionError {
    #[error("java.lang.ArithmeticException: / by zero")]
    ArithmeticDividedByZero,
}
