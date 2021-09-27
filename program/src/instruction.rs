use crate::error::GPTError::InvalidInstruction;
use solana_program::program_error::ProgramError;

pub enum GPTInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The wagerer account
    /// 1. `[writable]` The bet account of the wagerer
    /// 2. `[]` The PDA account for the bet type
    /// 2. `[]` The system program
    NewBet {
        bump_seed: u8,
    },
}

impl GPTInstruction {
    /// Unpacks a byte buffer into a GPTInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::NewBet {
                bump_seed: rest[0],
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
