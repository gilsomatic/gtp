use crate::{error::GPTError::InvalidInstruction, error::GPTError::InvalidBetType, state::BetType};
use solana_program::program_error::ProgramError;
use num_traits::FromPrimitive;
use std::convert::TryInto;

pub enum GPTInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The wagerer account
    /// 1. `[writable]` The PDA account for the bet type
    /// 2. `[writable]` The bet account of the wagerer
    /// 3. `[]` The system program
    NewBet {
        bet_type: BetType,
        bump_seed: u8,
        guess: u64,
    },
}

impl GPTInstruction {
    /// Unpacks a byte buffer into a GPTInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::NewBet {
                bet_type: FromPrimitive::from_u8(*rest.get(0).ok_or(InvalidInstruction)?).ok_or(InvalidBetType)?,
                bump_seed: *rest.get(1).ok_or(InvalidInstruction)?,
                guess: Self::unpack_guess(rest.get(2..).ok_or(InvalidInstruction)?)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_guess(input: &[u8]) -> Result<u64, ProgramError> {
        let guess = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(guess)
    }
}
