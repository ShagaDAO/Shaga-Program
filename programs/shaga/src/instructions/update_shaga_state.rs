use crate::{seeds::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateShagaState<'info> {
    #[account(mut)]
    pub shaga_authority: Signer<'info>,
    #[account(mut, has_one = shaga_authority, seeds = [SEED_SHAGA_STATE], bump)]
    pub shaga_state: Account<'info, ShagaState>,
    pub system_program: Program<'info, System>,
}

/// creates an affair by the lender/pc owner/creator.
pub fn handle_update_shaga_state(
    ctx: Context<UpdateShagaState>,
    new_shaga_authority: Option<Pubkey>,
    fee_destination: Option<Pubkey>,
    fee_basis_points: Option<u32>,
    is_paused: Option<bool>,
) -> Result<()> {
    let shaga_state = &mut ctx.accounts.shaga_state;

    if let Some(shaga_authority) = new_shaga_authority {
        shaga_state.shaga_authority = shaga_authority;
    }
    if let Some(fee_destination) = fee_destination {
        shaga_state.fee_destination = fee_destination;
    }

    if let Some(fee_basis_points) = fee_basis_points {
        shaga_state.fee_basis_points = fee_basis_points;
    }

    if let Some(is_paused) = is_paused {
        shaga_state.is_paused = is_paused;
    }

    Ok(())
}
