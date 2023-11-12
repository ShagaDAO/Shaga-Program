use crate::{seeds::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = ShagaState::size(), seeds = [SEED_SHAGA_STATE], bump)]
    pub shaga_state: Account<'info, ShagaState>,
    #[account(init, payer=payer, space = AffairsList::size(), seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    /// The pda that will own and manage threads.
    /// CHECK: safe because it is creating an predetermined signer
    #[account(init, payer=payer, space = 1, seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(
    ctx: Context<Initialize>,
    shaga_authority: Pubkey,
    fee_destination: Pubkey,
    fee_basis_points: u32,
    is_paused: bool,
) -> Result<()> {
    let shaga_state = &mut ctx.accounts.shaga_state;

    shaga_state.set_inner(ShagaState {
        is_paused,
        shaga_authority,
        fee_destination,
        fee_basis_points,
    });

    Ok(())
}
