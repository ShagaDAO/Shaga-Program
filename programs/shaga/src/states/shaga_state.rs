use crate::seeds::SEED_SHAGA_STATE;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct ShagaState {
    pub is_paused: bool,
    // ShagaDAO
    pub shaga_authority: Pubkey,
    // ShagaDAO (can be the same as above but we want to be able to be flexible)
    pub fee_destination: Pubkey,
    // shaga fee in basis points 1% == 100, 100% == 10_000.
    pub fee_basis_points: u32,
}

impl ShagaState {
    pub fn size() -> usize {
        8 + 1 + 32 + 32 + 1 + 4 + 200
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_SHAGA_STATE], &crate::ID)
    }
}
