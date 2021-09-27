use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum GPTError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid Bet Type")]
    InvalidBetType,
}

impl From<GPTError> for ProgramError {
    fn from(e: GPTError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
