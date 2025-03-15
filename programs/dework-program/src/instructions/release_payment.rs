use std::{sync::Mutex, thread::AccessError};

use crate::state::{job::*, quotes::*};
use anchor_lang::{prelude::*, system_program};

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Quote does not belong to this job")]
    InvalidQuote,

    #[msg("Job not marked commplete")]
    InvalidPaymentRequest,
}

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    #[account(mut, has_one = client @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account(mut, has_one = job @ ErrorCode::InvalidQuote)]
    pub quote: Account<'info, Quote>,

    #[account(
        mut,
        seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(mut, address = quote.worker)]
    pub worker_account: SystemAccount<'info>,

    #[account()]
    pub client: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let quote = &mut ctx.accounts.quote;
    let escrow_account = &mut ctx.accounts.escrow_account;

    if !job.job_completed {
        return Err(ErrorCode::InvalidPaymentRequest.into());
    }
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: escrow_account.to_account_info(),
                to: ctx.accounts.worker_account.to_account_info(),
            },
        ),
        quote.proposed_budget,
    )?;

    Ok(())
}
