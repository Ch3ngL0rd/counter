use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum CounterError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Overflow error!")]
    OverflowError,
    #[error("Rent Exemption Error!")]
    NotRentExempt,
    #[error("Unauthorised Error")]
    IncorrectUser,
}

// Converts our Counter Error into a Program Error (for solana_program::program_error)
impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}