use anchor_lang::prelude::*;
use crate::state::{job::*, quotes::*};
use crate::instructions::errors::ErrorCode;

#[derive(Accounts)]
pub struct ResolveDispute<'info>{

    #[account(mut, has_one = arbitrator @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account(mut, has_one = job @ ErrorCode::InvalidQuote)]
    pub quote: Account<'info, Quote>,

    /// CHECK: This is a manually derived PDA and is safe because it follows the escrow pattern.
    #[account(
        mut,
        seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()],
        bump,
        constraint = escrow_account.lamports() >= quote.proposed_budget @ ErrorCode::InsufficientFunds
    )]
    pub escrow_account: UncheckedAccount<'info>,
    
    #[account(mut, address = job.client)]
    pub client_account: SystemAccount<'info>,
    
    #[account(mut, address = quote.worker)]
    pub worker_account: SystemAccount<'info>,
    
    pub arbitrator: Signer<'info>,
    
    pub system_program: Program<'info, System>,

}

pub fn resolve_dispute(ctx: Context<ResolveDispute>, resolution: u8, split: Option<u8>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let quote = &mut ctx.accounts.quote;
    let escrow_account = &mut ctx.accounts.escrow_account;
    let client_account = &ctx.accounts.client_account;
    let worker_account = &ctx.accounts.worker_account;
    let amount = quote.proposed_budget;

    // Verify job is in dispute state
    require!(job.dispute_flag, ErrorCode::JobNotInDispute);
    
    // Verify escrow has sufficient funds
    require!(
        escrow_account.lamports() >= amount,
        ErrorCode::InsufficientFunds
    );

    // Handle resolution based on the option
    match resolution {
        0 => {
            // Full refund to client
            **escrow_account.try_borrow_mut_lamports()? -= amount;
            **client_account.to_account_info().try_borrow_mut_lamports()? += amount;
        },
        1 => {
            // Full payment to worker
            **escrow_account.try_borrow_mut_lamports()? -= amount;
            **worker_account.to_account_info().try_borrow_mut_lamports()? += amount;
        },
        2 => {
            // Split payment between client and worker
            let split_ratio = split.unwrap_or(50) as u64;
            require!(split_ratio <= 100, ErrorCode::InvalidResolutionOption);
            
            // Calculate shares safely
            let worker_share = (split_ratio * amount) / 100;
            let client_share = amount - worker_share;
            
            // Transfer client share
            **escrow_account.try_borrow_mut_lamports()? -= client_share;
            **client_account.to_account_info().try_borrow_mut_lamports()? += client_share;
            
            // Transfer worker share
            **escrow_account.try_borrow_mut_lamports()? -= worker_share;
            **worker_account.to_account_info().try_borrow_mut_lamports()? += worker_share;
        },
        _ => return Err(ErrorCode::InvalidResolutionOption.into()),
    };

    // Update job state
    job.dispute_flag = false;
    job.dispute_resolved = true;  // Add this field to Job struct
    job.dispute_resolved_at = Clock::get()?.unix_timestamp;  // Add this field to Job struct
    job.escrow_funded = false;  // Mark escrow as no longer funded
    job.payment_released = true;

    // Update quote state
    quote.dispute_resolution = Some(resolution);  // Add this field to Quote struct
    
    // Close escrow account and reclaim rent
    let remaining_balance = escrow_account.lamports();
    if remaining_balance > 0 {
        **escrow_account.try_borrow_mut_lamports()? = 0;
        **client_account.to_account_info().try_borrow_mut_lamports()? += remaining_balance;
    }

    Ok(())
}
