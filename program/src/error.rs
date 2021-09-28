use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum GPTError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid Bet Type")]
    InvalidBetType,
    #[error("Bet Is Close")]
    BetIsClose,
    #[error("Invalid PDA Account")]
    InvalidPDAAccount,
    #[error("Account Is Not Rent Exempt")]
    NotRentExempt,
    #[error("Bet Lamports Are Not Enough")]
    BetLamportNotEnough,
    #[error("The Bet Account Contains Data")]
    BetAccountContainsData,
}

impl From<GPTError> for ProgramError {
    fn from(e: GPTError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
