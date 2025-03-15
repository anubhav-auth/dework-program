use crate::state::{job::*, quotes::*};
use anchor_lang::{prelude::*, system_program};

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Quote does not belong to this job")]
    InvalidQuote,

    #[msg("Job not marked complete")]
    InvalidPaymentRequest,

    #[msg("At least 2 approvals required to release payment")]
    InsufficientApprovals,
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

    #[account(mut, address = quote.client)]
    pub client: SystemAccount<'info>,

    #[account()]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let quote = &mut ctx.accounts.quote;
    let escrow_account = &mut ctx.accounts.escrow_account;

    if !job.job_completed {
        return Err(ErrorCode::InvalidPaymentRequest.into());
    }

    if ctx.accounts.signer.key() == job.client {
        job.client_signed = true;
    } else if ctx.accounts.signer.key() == job.worker {
        job.worker_signed = true;
    } else if ctx.accounts.signer.key() == job.arbitrator {
        job.arbitrator_signed = true;
    } else {
        return Err(ErrorCode::Unauthorized.into());
    }


    let mut approval_count = 0;
    if job.client_signed { approval_count += 1; }
    if job.worker_signed { approval_count += 1; }
    if job.arbitrator_signed { approval_count += 1; }

    require!(approval_count >= 2, ErrorCode::InsufficientApprovals);

    let recipient = if job.client_signed && job.worker_signed {
        ctx.accounts.worker_account.to_account_info()
    } else if job.client_signed && job.arbitrator_signed {
        ctx.accounts.client.to_account_info()
    } else {
        ctx.accounts.worker_account.to_account_info()
    };

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: escrow_account.to_account_info(),
                to: recipient,
            },
        ),
        quote.proposed_budget,
    )?;

    Ok(())
}
