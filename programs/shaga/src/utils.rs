use crate::{seeds::*, ID};
use anchor_lang::prelude::*;

pub fn get_thread_id(thread_authority: &AccountInfo<'_>, account_key: &Pubkey) -> Vec<u8> {
    let (thread_id, _bump) = Pubkey::find_program_address(
        &[
            SEED_THREAD,
            thread_authority.key().as_ref(),
            account_key.as_ref(),
        ],
        &ID,
    );
    let thread_id_vec: Vec<u8> = thread_id.to_bytes().to_vec();
    thread_id_vec
}
