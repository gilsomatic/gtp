use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::GPTError::InvalidInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[repr(u8)]
pub enum BetType {
    SolUsd = 0,
    // others
}


// TODO: prova a vedere se puoi implementare la serializzazione con Borsh trasformando l'enum e strutture in or

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GPTInstruction {
    /// Accounts expected:
    ///
    /// 0. `[signer]` The wagerer account
    /// 1. `[]` The system program 0x11111111111111111111111111111111
    NewBet {
        bet_type: BetType,
        bump_seed: u8,
        guess: u64,
    },

    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade
    /// 1. `[writable]` The taker's token account for the token they send
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
    /// 4. `[writable]` The initializer's main account to send their rent fees to
    /// 5. `[writable]` The initializer's token account that will receive tokens
    /// 6. `[writable]` The escrow account holding the escrow info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    Exchange {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        amount: u64,
    },
}

impl GPTInstruction {
    /// Unpacks a byte buffer into a GPTInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::NewBet {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
