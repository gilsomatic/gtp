use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    native_token::sol_to_lamports,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::GPTError,
    instruction::GPTInstruction,
    state::ProgramAccount,
    state::{BetAccount, BetType},
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GPTInstruction::unpack(instruction_data)?;

        match instruction {
            GPTInstruction::NewBet {
                bet_type,
                bump_seed,
                guess,
            } => {
                msg!("Instruction: NewBet Type: {:?}", bet_type);
                Self::process_new_bet(accounts, bet_type, bump_seed, guess, program_id)
            }
        }
    }

    /// 0. `[signer]` The wagerer account
    /// 1. `[writable]` The PDA account for the bet type
    /// 2. `[writable]` The bet account of the wagerer
    /// 3. `[]` The system program
    fn process_new_bet(
        accounts: &[AccountInfo],
        bet_type: BetType,
        bump_seed: u8,
        guess: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        // Validate the wagerer account as a signer
        let wagerer = next_account_info(account_info_iter)?;
        if !wagerer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Validate the PDA account
        let pda_account = next_account_info(account_info_iter)?;
        if *pda_account.owner != *program_id {
            return Err(GPTError::InvalidPDAAccount.into());
        }
        // b"..." Byte string literal; constructs a [u8] instead of a string
        let seed = match bet_type {
            BetType::SolUsd => b"solusd",
        };
        let pda_expected_key = Pubkey::create_program_address(&[seed, &[bump_seed]], program_id)?;
        if *pda_account.key != pda_expected_key {
            return Err(GPTError::InvalidPDAAccount.into());
        }
        let rent = Rent::get()?;
        if !rent.is_exempt(pda_account.lamports(), pda_account.data_len()) {
            return Err(GPTError::NotRentExempt.into());
        }

        // Validate the bet account
        let bet_account = next_account_info(account_info_iter)?;
        if bet_account.lamports() < sol_to_lamports(0.1) {
            return Err(GPTError::BetLamportNotEnough.into());
        }
        if bet_account.data_len() > 0 {
            return Err(GPTError::BetAccountContainsData.into());
        } // TODO: disaccoppio il client allocando la data space direttamente qui, così il client non deve sapere
          // come è fatta la struttura interna ma passa solo i parametri e un account con 0.1 lamport

        let system_program = next_account_info(account_info_iter)?;

        /* Runtime policy: Only the owner of the account may change owner.
            - And only if the account is writable.
            - And only if the account is not executable
            - And only if the data is zero-initialized or empty.
        */
        let owner_change_ix = system_instruction::assign(bet_account.key, pda_account.key);
        /*
        invoke(
            &owner_change_ix,
            &[
                takers_sending_token_account.clone(),
                initializers_token_to_receive_account.clone(),
                taker.clone(),
                token_program.clone(),
            ],
        )?;
        */

        /*
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;
        */

        /*
        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(GPTError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.wagerer_pubkey = *wagerer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.wagerer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            wagerer.key,
            &[&wagerer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                wagerer.clone(),
                token_program.clone(),
            ],
        )?;
        */

        Ok(())
    }

    /*
    fn process_exchange(
        accounts: &[AccountInfo],
        amount_expected_by_taker: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let taker = next_account_info(account_info_iter)?;

        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let takers_sending_token_account = next_account_info(account_info_iter)?;

        let takers_token_to_receive_account = next_account_info(account_info_iter)?;

        let pdas_temp_token_account = next_account_info(account_info_iter)?;
        let pdas_temp_token_account_info =
            TokenAccount::unpack(&pdas_temp_token_account.data.borrow())?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        if amount_expected_by_taker != pdas_temp_token_account_info.amount {
            return Err(GPTError::ExpectedAmountMismatch.into());
        }

        let wagerers_main_account = next_account_info(account_info_iter)?;
        let wagerers_token_to_receive_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;

        let escrow_info = Escrow::unpack(&escrow_account.data.borrow())?;

        if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.wagerer_pubkey != *wagerers_main_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.wagerer_token_to_receive_account_pubkey
            != *wagerers_token_to_receive_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let token_program = next_account_info(account_info_iter)?;

        let transfer_to_wagerer_ix = spl_token::instruction::transfer(
            token_program.key,
            takers_sending_token_account.key,
            wagerers_token_to_receive_account.key,
            taker.key,
            &[&taker.key],
            escrow_info.expected_amount,
        )?;
        msg!("Calling the token program to transfer tokens to the escrow's wagerer...");
        invoke(
            &transfer_to_wagerer_ix,
            &[
                takers_sending_token_account.clone(),
                wagerers_token_to_receive_account.clone(),
                taker.clone(),
                token_program.clone(),
            ],
        )?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_to_taker_ix = spl_token::instruction::transfer(
            token_program.key,
            pdas_temp_token_account.key,
            takers_token_to_receive_account.key,
            &pda,
            &[&pda],
            pdas_temp_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the taker...");
        invoke_signed(
            &transfer_to_taker_ix,
            &[
                pdas_temp_token_account.clone(),
                takers_token_to_receive_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pdas_temp_token_account.key,
            wagerers_main_account.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pdas_temp_token_account.clone(),
                wagerers_main_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        msg!("Closing the escrow account...");
        **wagerers_main_account.lamports.borrow_mut() = wagerers_main_account
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(GPTError::AmountOverflow)?;
        **escrow_account.lamports.borrow_mut() = 0;
        *escrow_account.data.borrow_mut() = &mut [];

        Ok(())
    }
    */
}
