use crate::{checks::*, errors::*, seeds::*, states::*, utils::*, ID};
use anchor_lang::prelude::*;
use anchor_lang::InstructionData;
use clockwork_sdk::cpi::thread_create;
use solana_program::instruction::Instruction;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AffairPayload {
    pub coordinates: String,
    pub ip_address: String,
    pub cpu_name: String,
    pub gpu_name: String,
    pub total_ram_mb: u32,
    pub sol_per_hour: u64,
    pub affair_termination_time: u64,
    pub hash_algorithm: HashAlgorithm,
    pub private_pair_hash: Option<Vec<u8>>,
}

impl Default for AffairPayload {
    fn default() -> Self {
        Self {
            coordinates: "".to_string(),
            ip_address: "".to_string(),
            cpu_name: "".to_string(),
            gpu_name: "".to_string(),
            total_ram_mb: 0,
            sol_per_hour: 0,
            affair_termination_time: 0,
            hash_algorithm: HashAlgorithm::None,
            private_pair_hash: None,
        }
    }
}

#[derive(Accounts)]
pub struct CreateAffairAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [SEED_SHAGA_STATE], bump)]
    pub shaga_state: Account<'info, ShagaState>,
    #[account(init, payer = authority, space = Affair::size(), seeds = [SEED_AFFAIR, authority.key().as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, has_one = authority @ ShagaErrorCode::UnauthorizedAffairCreation, seeds = [SEED_LENDER, authority.key().as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    /// CHECK: checked below
    #[account(mut)]
    pub affair_clockwork_thread: SystemAccount<'info>,
    /// CHECK: checked below
    /// The pda that will own and manage the thread.
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
}

/// creates an affair by the lender/pc owner/creator.
pub fn handle_create_affair(
    ctx: Context<CreateAffairAccounts>,
    payload: AffairPayload,
) -> Result<()> {
    // Step 1: Initialize mutable references for Affair and Lender accounts
    let affair_account = &mut ctx.accounts.affair;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let shaga_state = &ctx.accounts.shaga_state;
    let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;
    let thread_authority = &ctx.accounts.thread_authority;
    let authority = &ctx.accounts.authority;
    let lender = &ctx.accounts.lender;
    let system_program = &ctx.accounts.system_program;
    let clockwork_program = &ctx.accounts.clockwork_program;

    // safety measure incase shaga pause is needed
    check_is_shaga_paused(shaga_state)?;
    // Step 2A: Populate it with payload and default values
    affair_account.authority = authority.key();
    affair_account.coordinates = payload.coordinates;
    affair_account.ip_address = payload.ip_address;
    affair_account.cpu_name = payload.cpu_name;
    affair_account.gpu_name = payload.gpu_name;
    affair_account.total_ram_mb = payload.total_ram_mb;
    affair_account.sol_per_hour = payload.sol_per_hour;
    affair_account.affair_termination_time = payload.affair_termination_time as u64;
    affair_account.affair_state = AffairState::Available;

    if payload
        .private_pair_hash
        .clone()
        .is_some_and(|private_hash| private_hash.len() != 32)
    {
        msg!("private_pair_hash must be 32 bytes long.");
        return Err(ShagaErrorCode::InvalidAffair.into());
    }

    affair_account.private_pair_hash = payload.private_pair_hash;
    affair_account.hash_algorithm = payload.hash_algorithm;

    let borrow_affair_account = affair_account.clone();
    // Step 2B: Accounts for terminate_affair instruction
    // Step 2C: Create the terminate_affair_instruction
    let target_ix = Instruction {
        program_id: ID,
        accounts: crate::__client_accounts_terminate_vacant_affair_accounts::TerminateVacantAffairAccounts {
            signer: affair_clockwork_thread.key(),
            authority: authority.key(),
            shaga_state: shaga_state.key(),
            lender: lender.key(),
            thread_authority: thread_authority.key(),
            affair: affair_account.key(),
            affairs_list: affairs_list_account.key(),
            affair_clockwork_thread: affair_clockwork_thread.key(),
            system_program: system_program.key(),
            clockwork_program: clockwork_program.key()
        }
        .to_account_metas(Some(true)),
        data: crate::instruction::TerminateVacantAffair {}.data(),
    };

    // Step 3: Fetch the current timestamp and validate affair termination time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    if affair_account.affair_termination_time <= current_time {
        msg!("Affair termination time must be in the future.");
        return Err(ShagaErrorCode::InvalidTerminationTime.into());
    }
    // , address = Thread::pubkey(thread_authority.key(), thread_id)
    // Step 5: Define thread_id with seeds & trigger for the termination thread
    let thread_id_vec = get_thread_id(thread_authority, &borrow_affair_account.key());
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: affair_account.affair_termination_time as i64,
    };

    // Step 6: Fetch the bump seed associated with the authority
    check_valid_clockword_key(
        thread_authority,
        &thread_id_vec,
        clockwork_program,
        &affair_clockwork_thread.key(),
    )?;

    let ta_bump = *ctx.bumps.get("thread_authority").unwrap();
    let cpi_signer: &[&[u8]] = &[SEED_AUTHORITY_THREAD, &[ta_bump]];
    let binding_seeds = &[cpi_signer];
    // Step 7: Create the termination thread
    let cpi_ctx = CpiContext::new_with_signer(
        clockwork_program.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: authority.to_account_info(),
            system_program: system_program.to_account_info(),
            thread: affair_clockwork_thread.to_account_info(),
            authority: thread_authority.to_account_info(),
        },
        binding_seeds,
    );

    // Execute the thread creation
    thread_create(
        cpi_ctx,
        1000, // clockwork MINIMUM_FEE
        thread_id_vec,
        vec![target_ix.into()],
        trigger,
    )?;

    // Step 9: Add Affair to Affair List
    let affair_pubkey = affair_account.key();

    // Register the affair
    if affair_account.private_pair_hash.is_none() {
        affairs_list_account.register_affair(affair_pubkey)?;
    }

    // Step 10: All steps successful, return Ok
    Ok(())
}
