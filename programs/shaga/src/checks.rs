use crate::{errors::*, states::*, SEED_THREAD};
use anchor_lang::prelude::*;

pub fn check_is_shaga_paused(shaga_state: &Account<'_, ShagaState>) -> Result<()> {
    if shaga_state.is_paused {
        msg!("Shaga is currently paused.");
        return Err(ShagaErrorCode::ShagaPaused.into());
    }
    Ok(())
}

pub fn check_can_start_rental(affair: &Account<'_, Affair>) -> Result<()> {
    if !affair.can_join() {
        msg!("The Affair is already rented");
        return Err(ShagaErrorCode::AffairAlreadyJoined.into());
    }
    Ok(())
}

pub fn check_sufficient_funds(client_lamports: u64, amount: u64) -> Result<()> {
    if client_lamports < amount {
        msg!("Insufficient funds.");
        return Err(ShagaErrorCode::InsufficientFunds.into());
    }
    Ok(())
}

pub fn check_valid_clockword_key(
    thread_authority: &AccountInfo<'_>,
    thread_id_vec: &Vec<u8>,
    clockwork_program: &Program<'_, clockwork_sdk::ThreadProgram>,
    clockwork_thread_key: &Pubkey,
) -> Result<()> {
    let (clockwork_thread_computed, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            thread_id_vec.as_slice().as_ref(),
        ],
        &clockwork_program.key(),
    );
    if clockwork_thread_computed.key() != *clockwork_thread_key {
        msg!("Invalid clockwork thread key.");
        return Err(ShagaErrorCode::ClockWorkKeyMismatch.into());
    }
    Ok(())
}
