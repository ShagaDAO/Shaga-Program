use crate::{checks::*, errors::*, seeds::*, states::*, utils::*};
use anchor_lang::prelude::*;

use clockwork_sdk::cpi::thread_delete;
use solana_program::{clock::Clock, native_token::Sol};

#[derive(Accounts)]
pub struct TerminateAffairAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: checked below. possibly none.
    #[account(mut)]
    pub client: SystemAccount<'info>,
    #[account(mut, seeds = [SEED_SHAGA_STATE], bump)]
    pub shaga_state: Account<'info, ShagaState>,
    #[account(mut, has_one = authority @ ShagaErrorCode::UnauthorizedAffairTerminator, seeds = [SEED_LENDER, affair.authority.as_ref()], bump)]
    pub lender: Account<'info, Lender>,
    #[account(mut, has_one = authority, seeds = [SEED_AFFAIR, authority.key().as_ref()], bump)]
    pub affair: Account<'info, Affair>,
    #[account(mut, seeds = [SEED_AFFAIR_LIST], bump)]
    pub affairs_list: Account<'info, AffairsList>,
    #[account(mut, seeds = [SEED_ESCROW, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, seeds = [SEED_RENTAL, lender.key().as_ref(), client.key().as_ref()], bump)]
    pub rental: Account<'info, Rental>,
    /// CHECK: checked below
    #[account(mut)]
    pub affair_clockwork_thread: UncheckedAccount<'info>,
    /// CHECK: checked below
    #[account(mut)]
    pub rental_clockwork_thread: UncheckedAccount<'info>,
    /// CHECK: checked with seeds
    #[account(seeds = [SEED_AUTHORITY_THREAD], bump)]
    pub thread_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
}

pub fn handle_affair_termination(ctx: Context<TerminateAffairAccounts>) -> Result<()> {
    let affair_account = &ctx.accounts.affair;
    let affairs_list_account = &mut ctx.accounts.affairs_list;
    let escrow_account = &mut ctx.accounts.escrow;
    let rental_account = &ctx.accounts.rental;
    let client = &ctx.accounts.client;
    let authority = &ctx.accounts.authority;
    let affair_clockwork_thread = &ctx.accounts.affair_clockwork_thread;
    let thread_authority = &ctx.accounts.thread_authority;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let rental_clockwork_thread = &ctx.accounts.rental_clockwork_thread;
    let lender = &mut ctx.accounts.lender;

    if affair_account.rental.is_none() {
        msg!("Invalid instruction there is no ongoing rental.");
        return Err(ShagaErrorCode::InvalidTerminationInstruction.into());
    }

    // TODO: figure out if we should delete the thread if the thread executed the instruction
    let borrow_affair_account = affair_account.clone();

    let thread_id_vec = get_thread_id(thread_authority, &borrow_affair_account.key());

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
        clockwork_sdk::cpi::ThreadDelete {
            authority: thread_authority.to_account_info(),
            close_to: authority.to_account_info(),
            thread: affair_clockwork_thread.to_account_info(),
        },
        binding_seeds,
    );

    thread_delete(cpi_ctx)?;

    // Remove the affair from the list of active affairs
    let affair_pubkey = affair_account.key();
    if affair_account.private_pair_hash.is_none() {
        affairs_list_account.remove_affair(affair_pubkey);
    }

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    if current_time >= rental_account.rental_termination_time {
        msg!("current time is higher than rental termination time. rental has ended.");
        let authority_account_info = &mut authority.to_account_info();
        let escrow_account_info = &mut escrow_account.to_account_info();

        let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let actual_payment = rental_account.rent_amount;
        **escrow_lamports -= actual_payment;
        **authority_lamports += actual_payment;

        msg!("actual_payment: {}", Sol(actual_payment));
        msg!("authority_lamports: {}", Sol(**authority_lamports));
        msg!("escrow_lamports: {}", Sol(**escrow_lamports));
        lender.total_earnings += actual_payment;
    } else {
        // terminate rental thread

        // TODO: figure out if we should delete the thread if the thread executed the instruction
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
        // Step 7: Create the termination thread
        let cpi_ctx = CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadDelete {
                authority: thread_authority.to_account_info(),
                close_to: client.to_account_info(),
                thread: rental_clockwork_thread.to_account_info(),
            },
            binding_seeds,
        );

        thread_delete(cpi_ctx)?;

        if affair_account.sol_per_hour > 0 {
            // using a factor of 100:
            let scaling_factor = 100_u64;

            let actual_time =
                (current_time as f64 - rental_account.rental_start_time as f64) / 3600.0;
            let scaled_rental_duration = (actual_time * scaling_factor as f64) as u64;
            let actual_payment = scaled_rental_duration
                .checked_mul(affair_account.sol_per_hour)
                .ok_or(ShagaErrorCode::NumericalOverflow)?
                .checked_div(scaling_factor)
                .ok_or(ShagaErrorCode::NumericalOverflow)?;

            let refund_amount = escrow_account
                .locked_amount
                .checked_sub(actual_payment)
                .ok_or(ShagaErrorCode::NumericalOverflow)?;

            let client_account_info = &mut client.to_account_info();
            let authority_account_info = &mut authority.to_account_info();
            let escrow_account_info = &mut escrow_account.to_account_info();

            let mut escrow_lamports = escrow_account_info.try_borrow_mut_lamports()?;
            let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
            let mut client_lamports = client_account_info.try_borrow_mut_lamports()?;

            **escrow_lamports -= refund_amount + actual_payment;
            **authority_lamports += actual_payment;
            **client_lamports += refund_amount;

            msg!("actual_payment: {}", Sol(actual_payment));
            msg!("refund_amount: {}", Sol(refund_amount));
            msg!("escrow_lamports: {}", Sol(**escrow_lamports));
            msg!("authority_lamports: {}", Sol(**authority_lamports));
            msg!("client_lamports: {}", Sol(**client_lamports));
            lender.give_thumbs_down();
            lender.total_earnings += actual_payment;
        }
    }

    // check if lender has some sols to retrieve.
    let lender_account_info = &mut ctx.accounts.lender.to_account_info();
    let lender_rent = Rent::get()?.minimum_balance(lender_account_info.data_len());
    let lender_balance = lender_account_info.lamports() - lender_rent;
    if lender_balance > 0 {
        let authority_account_info = &mut ctx.accounts.authority.to_account_info();
        let mut authority_lamports = authority_account_info.try_borrow_mut_lamports()?;
        let mut lender_lamports = lender_account_info.try_borrow_mut_lamports()?;

        **lender_lamports -= lender_balance;
        **authority_lamports += lender_balance;
    }

    // since rent ended and we already transfered the total.
    // we can close these accounts below.
    affair_account.close(authority.to_account_info())?;
    escrow_account.close(authority.to_account_info())?;
    rental_account.close(authority.to_account_info())?;

    Ok(())
}
