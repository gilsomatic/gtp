use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    pubkey::Pubkey,
};
use crate::{error::GPTError};
use num_derive::FromPrimitive;    
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(FromPrimitive, Debug, Clone, Copy)]
#[repr(u8)]
pub enum BetType {
    SolUsd = 0,
    // others
}

pub struct ProgramAccount {
    pub bet_type: BetType, // 1
    pub is_open: bool, // 1
    pub head_pubkey: Pubkey, // 32
    pub number_of_bettors: u32, // 4
}

impl Sealed for ProgramAccount {}

impl Pack for ProgramAccount {
    const LEN: usize = 38; // 1 + 1 + 32 + 4
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ProgramAccount::LEN];
        let (
            bet_type,
            is_open,
            head_pubkey,
            number_of_bettors,
        ) = array_refs![src, 1, 1, 32, 4];
        let bet_type = match bet_type {
            [0] => BetType::SolUsd,
            _ => return Err(GPTError::InvalidBetType.into()),
        };
        let is_open = match is_open {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(ProgramAccount {
            bet_type,
            is_open,
            head_pubkey: Pubkey::new_from_array(*head_pubkey),
            number_of_bettors: u32::from_le_bytes(*number_of_bettors),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ProgramAccount::LEN];
        let (
            bet_type_dst,
            is_open_dst,
            head_pubkey_dst,
            number_of_bettors_dst,
        ) = mut_array_refs![dst, 1, 1, 32, 4];

        let ProgramAccount {
            bet_type,
            is_open,
            head_pubkey,
            number_of_bettors,
        } = self;

        bet_type_dst[0] = *bet_type as u8;
        is_open_dst[0] = *is_open as u8;
        head_pubkey_dst.copy_from_slice(head_pubkey.as_ref());
        *number_of_bettors_dst = number_of_bettors.to_le_bytes();
    }
}

pub struct BetAccount {
    pub bet_type: BetType, // 1
    pub guess: u64, // 8
    pub time_slot: u64, // 8
    pub next_bet_pubkey: Pubkey, // 32
    pub bettor_pubkey: Pubkey, // 32
}

impl Sealed for BetAccount {}

impl Pack for BetAccount {
    const LEN: usize = 81; // 1 + 8 + 8 + 32 + 32
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, BetAccount::LEN];
        let (
            bet_type,
            guess,
            time_slot,
            next_bet,
            bettor,
        ) = array_refs![src, 1, 8, 8, 32, 32];
        let bet_type = match bet_type {
            [0] => BetType::SolUsd,
            _ => return Err(GPTError::InvalidBetType.into()),
        };

        Ok(BetAccount {
            bet_type,
            guess: u64::from_le_bytes(*guess),
            time_slot: u64::from_le_bytes(*time_slot),
            next_bet_pubkey: Pubkey::new_from_array(*next_bet),
            bettor_pubkey: Pubkey::new_from_array(*bettor),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, BetAccount::LEN];
        let (
            bet_type_dst,
            guess_dst,
            time_slot_dst,
            next_bet_dst,
            bettor_dst,
        ) = mut_array_refs![dst, 1, 8, 8, 32, 32];

        let BetAccount {
            bet_type,
            guess,
            time_slot,
            next_bet_pubkey,
            bettor_pubkey,
        } = self;

        bet_type_dst[0] = *bet_type as u8;
        *guess_dst = guess.to_le_bytes();
        *time_slot_dst = time_slot.to_le_bytes();
        next_bet_dst.copy_from_slice(next_bet_pubkey.as_ref());
        bettor_dst.copy_from_slice(bettor_pubkey.as_ref());
    }
}
