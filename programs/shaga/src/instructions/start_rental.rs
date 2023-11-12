use crate::instructions::RentalTerminationAuthority;
use crate::{checks::*, errors::*, seeds::*, states::*, utils::*, ID};
use anchor_lang::prelude::*;
use anchor_lang::InstructionData;

use solana_program::instruction::Instruction;
use solana_program::native_token::Sol;

#[derive(Accounts)]
pub struct StartRentalAccounts<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut, seeds = [SEED_SHAGA_STATE], bump)]
    pub shaga_state: Account<'info, ShagaState>,
    #[account(mut, seeds = [SEED_LENDER, affair.authority.as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    #[account(mut, seeds = [SEED_AFFAIR, affair.authority.as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(init, payer = client, space = Escrow::size(), seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(init, payer = client, space = Rental::size(), seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    /// CHECK: checked in shaga_state
    #[account(mut)]
    pub fee_destination: UncheckedAccount<'info>,
    /// CHECK: checked below
    #[account(mut)]
    pub rental_clockwork_thread: UncheckedAccount<'info>,
    /// CHECK: via seeds
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
}

pub fn handle_starting_rental(
    ctx: Context<StartRentalAccounts>,
    rental_termination_time: u64,
    private_pair_hash_code: Option<String>,
) -> Result<()> {
    let affair_account = &mut ctx.accounts.affair;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &mut ctx.accounts.rental;
    let shaga_state = &ctx.accounts.shaga_state;
    let fee_destination = &ctx.accounts.fee_destination;
    let client_account = &ctx.accounts.client;
    let lender = &mut ctx.accounts.lender;
    let affairs_list = &mut ctx.accounts.affairs_list;
    let thread_authority = &ctx.accounts.thread_authority;
    let rental_clockwork_thread = &ctx.accounts.rental_clockwork_thread;
    let system_program = &ctx.accounts.system_program;
    let clockwork_program = &ctx.accounts.clockwork_program;

    // safety measure incase shaga pause is needed
    check_is_shaga_paused(shaga_state)?;
    // Step 2: Validate if the affair can be joined
    check_can_start_rental(affair_account)?;

    // check if private affair hash exists
    if affair_account.private_pair_hash.is_some() {
        // if the ixn does not include private hash code then something is wrong
        if private_pair_hash_code.is_none() {
            msg!("Private pair code is missing.");
            return Err(ShagaErrorCode::MissingPrivatePairCode.into());
        }

        // we use sha256 for now but when blake3 is live on mainnet we can migrate it to it.
        let private_pair_hash_code_unwrapped = private_pair_hash_code.unwrap();
        affair_account.verify_private_pair_hash(&private_pair_hash_code_unwrapped)?;
    }

    // Step 3: Validate rental_termination_time
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    if rental_termination_time > affair_account.affair_termination_time
        || rental_termination_time <= current_time as u64
    {
        msg!("Invalid rental termination time.");
        return Err(ShagaErrorCode::InvalidRentalTerminationTime.into());
    }

    if affair_account.sol_per_hour > 0 {
        // Step 4: Calculate rent cost & fee amount
        // using a factor of 100:
        let scaling_factor = 100.0;

        let rental_duration_seconds = rental_termination_time
            .checked_sub(current_time)
            .ok_or(ShagaErrorCode::NumericalOverflow)?;

        let rental_duration_hours_float = rental_duration_seconds as f64 / 3600.0;
        let scaled_rental_duration = rental_duration_hours_float * scaling_factor;
        let rent_amount_scaled = scaled_rental_duration * affair_account.sol_per_hour as f64;

        let rent_amount = (rent_amount_scaled / scaling_factor) as u64;
        // Initialize fee_amount
        let mut fee_amount: u64 = 0;

        // Calculate fee_amount only if fee_basis_points is set
        if shaga_state.fee_basis_points > 0 {
            fee_amount = (shaga_state.fee_basis_points as u128)
                .checked_mul(rent_amount as u128)
                .ok_or(ShagaErrorCode::NumericalOverflow)?
                .checked_div(10_000)
                .ok_or(ShagaErrorCode::NumericalOverflow)? as u64;

            // Log the fee amount
            msg!("fee_amount: {}", Sol(fee_amount));
        } else {
            msg!("no fee!");
        }

        // Log the rent amount
        msg!("rent_amount: {}", Sol(rent_amount));

        // Step 4A: Check if the client's account has enough balance in terms of Lamports
        check_sufficient_funds(client_account.lamports(), (rent_amount + fee_amount) as u64)?;

        // Proceed with transfers only if fee_amount is non-zero
        if fee_amount > 0 {
            // Step 5: Transfer fee to the fee_destination
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    client_account.key,
                    &fee_destination.key(),
                    fee_amount,
                ),
                &[
                    client_account.to_account_info().clone(),
                    fee_destination.to_account_info().clone(),
                    system_program.to_account_info().clone(),
                ],
            )?;
        }

        // Step 6: Transfer the rent (minus any fee) to the escrow
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                client_account.key,
                &escrow_account.key(),
                (rent_amount - fee_amount) as u64,
            ),
            &[
                client_account.to_account_info().clone(),
                escrow_account.to_account_info().clone(),
                system_program.to_account_info().clone(),
            ],
        )?;

        // Step 6A: Update locked amount flag in Escrow
        escrow_account.locked_amount = (rent_amount - fee_amount) as u64;
    }
    // Step 7: Mark the affair as joined by setting the rental account pubkey
    affair_account
        .join(rental_account.key())
        .expect("Failed to start rental");
    // Step 8: Initialize the Rental account
    rental_account.initialize(
        client_account.key(),
        affair_account.key(),
        escrow_account.locked_amount,
        current_time as u64,
        rental_termination_time,
        rental_clockwork_thread.key(),
    );

    // Step 9A: Accounts for instruction
    let target_ix = Instruction {
        program_id: ID,
        accounts: crate::__client_accounts_end_rental_accounts::EndRentalAccounts {
            signer: rental_clockwork_thread.key(),
            client: client_account.key(),
            shaga_state: shaga_state.key(),
            escrow: escrow_account.key(),
            rental: rental_account.key(),
            thread_authority: thread_authority.key(),
            lender: lender.key(),
            affair: affair_account.key(),
            affairs_list: affairs_list.key(),
            rental_clockwork_thread: rental_clockwork_thread.key(),
            system_program: system_program.key(),
            clockwork_program: clockwork_program.key(),
        }
        .to_account_metas(Some(true)),
        data: crate::instruction::EndRental {
            termination_by: RentalTerminationAuthority::Clockwork,
        }
        .data(),
    };
    // Step 9C: Thread Trigger & Thread_ID
    let trigger = clockwork_sdk::state::Trigger::Timestamp {
        unix_ts: rental_termination_time as i64,
    };
    let thread_id_vec = get_thread_id(thread_authority, &rental_account.key());

    check_valid_clockword_key(
        thread_authority,
        &thread_id_vec,
        clockwork_program,
        &rental_clockwork_thread.key(),
    )?;

    let ta_bump = *ctx.bumps.get("thread_authority").unwrap();
    let cpi_signer: &[&[u8]] = &[SEED_AUTHORITY_THREAD, &[ta_bump]];
    let binding_seeds = &[cpi_signer];

    let cpi_ctx = CpiContext::new_with_signer(
        clockwork_program.to_account_info(),
        clockwork_sdk::cpi::ThreadCreate {
            payer: client_account.to_account_info(),
            system_program: system_program.to_account_info(),
            thread: rental_clockwork_thread.to_account_info(),
            authority: thread_authority.to_account_info(),
        },
        binding_seeds,
    );
    clockwork_sdk::cpi::thread_create(
        cpi_ctx,       // Use the CPI context you've created
        1000,          // MINIMUM_FEE
        thread_id_vec, // Use the converted thread_id
        vec![target_ix.into()],
        trigger,
    )?;
    msg!("thread created");

    // Step 9B: Save the end_rental thread account in the rental account
    rental_account.rental_clockwork_thread = rental_clockwork_thread.key();
    rental_account.rental_start_time = current_time;
    rental_account.rent_amount = escrow_account.locked_amount;

    // Step 10: increment affairs
    lender.increment_affairs();

    // Step 11: Associate the rental account with the affair
    affair_account.rental = Some(rental_account.key());

    // Step 12: Remove Affair from Affair List
    let affair_pubkey = affair_account.key();

    if affair_account.private_pair_hash.is_none() {
        affairs_list.remove_affair(affair_pubkey);
    }

    // Step 13: Update the Affair account
    affair_account.client = client_account.key();
    affair_account.active_rental_start_time = current_time;
    affair_account.due_rent_amount = escrow_account.locked_amount;
    //affair_account.active_locked_amount = (rent_amount - fee_amount) as u64;

    Ok(())
}

/*
#[derive(Accounts)]
pub struct InitializeThread<'info> {
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: AccountInfo<'info>,
    #[account(mut, address = solana_program::pubkey::Pubkey(thread_authority.key().to_bytes().to_vec(), active_rental.key().to_bytes().to_vec()))]
    pub thread: Account<'info, clockwork_sdk::state::Thread>,
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: Account<'info, clockwork_sdk::state::Thread>,
    pub active_rental: AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(thread_id: Vec<u8>)]
pub struct StartThread<'info> {
    #[account(
    seeds = [b"highscore_list_v2".as_ref()],
    bump,
    )]
    pub highscore: Account<'info, Highscore>,
    #[account(
    seeds = [b"price_pool".as_ref()],
    bump,
    )]
    pub price_pool: Account<'info, Pricepool>,
    /// The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    /// The signer who will pay to initialize the program.
    /// (not to be confused with the thread executions).
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    /// Address to assign to the newly created thread.
    #[account(mut, address = Thread::pubkey(thread_authority.key(), thread_id))]
    pub thread: SystemAccount<'info>,
    /// The pda that will own and manage the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
}
*/
