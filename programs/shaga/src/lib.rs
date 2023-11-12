// #![feature(is_some_and)]
use anchor_lang::prelude::*;
pub mod checks;
pub mod errors;
pub mod instructions;
pub mod seeds;
pub mod states;
pub mod utils;
pub use {checks::*, errors::*, instructions::*, seeds::*, states::*, utils::*};

declare_id!("HQeckNoXMczA5AtgKKWmLzQPT4Wcm6YBjeHCrRp2XLF1");

#[program]
pub mod shaga {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        shaga_authority: Pubkey,
        fee_destination: Pubkey,
        fee_basis_points: u32,
        is_paused: bool,
    ) -> Result<()> {
        handle_initialize(
            ctx,
            shaga_authority,
            fee_destination,
            fee_basis_points,
            is_paused,
        )
    }

    pub fn initialize_lender(ctx: Context<InitializeLender>) -> Result<()> {
        create_lender::handle_lender_initialization(ctx)
    }

    pub fn create_affair(ctx: Context<CreateAffairAccounts>, payload: AffairPayload) -> Result<()> {
        create_affair::handle_create_affair(ctx, payload)
    }

    pub fn start_rental(
        ctx: Context<StartRentalAccounts>,
        rental_termination_time: u64,
        private_pair_hash_code: Option<String>,
    ) -> Result<()> {
        start_rental::handle_starting_rental(ctx, rental_termination_time, private_pair_hash_code)
    }

    pub fn end_rental(
        ctx: Context<EndRentalAccounts>,
        termination_by: RentalTerminationAuthority,
    ) -> Result<()> {
        end_rental::handle_ending_rental(ctx, termination_by)
    }

    pub fn terminate_affair(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
        terminate_affair::handle_affair_termination(ctx)
    }
    /// handled by clockwork
    pub fn terminate_vacant_affair(ctx: Context<TerminateVacantAffairAccounts>) -> Result<()> {
        terminate_vacant_affair::handle_vacant_affair_termination(ctx)
    }

    pub fn update_shaga_state(
        ctx: Context<UpdateShagaState>,
        new_shaga_authority: Option<Pubkey>,
        fee_destination: Option<Pubkey>,
        fee_basis_points: Option<u32>,
        is_paused: Option<bool>,
    ) -> Result<()> {
        handle_update_shaga_state(
            ctx,
            new_shaga_authority,
            fee_destination,
            fee_basis_points,
            is_paused,
        )
    }
}

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Shaga Program",
    project_url: "https://shaga.xyz/",
    contacts: "email:team@shaga.xyz,link:https://web3shaga.gitbook.io/shaga/,discord:https://discord.gg/G5c3UwMfbH",
    policy: "https://github.com/ShagaDAO/Shaga-Program/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/ShagaDAO/Shaga-Program"
}
