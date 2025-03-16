use crate::state::{job::*, quotes::*};
use anchor_lang::prelude::*;
use crate::instructions::errors::ErrorCode;

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    #[account(mut, has_one = client @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account(mut, has_one = job @ ErrorCode::InvalidQuote)]
    pub quote: Account<'info, Quote>,

    #[account(
        mut,
        seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()],
        bump,
        constraint = escrow_account.lamports() >= quote.proposed_budget @ ErrorCode::InsufficientFunds
    )]
    /// CHECK: This is a manually derived PDA and is safe because it follows the escrow pattern.
    pub escrow_account: UncheckedAccount<'info>,

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

    require!(
        escrow_account.lamports() >= quote.proposed_budget,
        ErrorCode::InsufficientFunds
    );

    if ctx.accounts.signer.key() == job.client && !job.client_signed {
        job.client_signed = true;
        job.signatures += 1;
    } else if ctx.accounts.signer.key() == job.worker && !job.worker_signed {
        job.worker_signed = true;
        job.signatures += 1;
    } else if ctx.accounts.signer.key() == job.arbitrator && !job.arbitrator_signed {
        job.arbitrator_signed = true;
        job.signatures += 1;
    } else if ctx.accounts.signer.key() != job.client && 
              ctx.accounts.signer.key() != job.worker && 
              ctx.accounts.signer.key() != job.arbitrator {
        return Err(ErrorCode::Unauthorized.into());
    }


    let mut approval_count = 0;
    if job.client_signed { approval_count += 1; }
    if job.worker_signed { approval_count += 1; }
    if job.arbitrator_signed { approval_count += 1; }

    require!(approval_count >= 2, ErrorCode::InsufficientApprovals);

    // Determine recipient based on signatures
    // Updated logic: if worker signed, they get the payment unless there's a dispute
    let recipient = if job.worker_signed && job.client_signed {
        // Normal case: worker and client agree
        ctx.accounts.worker_account.to_account_info()
    } else if job.worker_signed && job.arbitrator_signed {
        // Worker and arbitrator agree (even without client)
        ctx.accounts.worker_account.to_account_info()
    } else if job.client_signed && job.arbitrator_signed {
        // Client and arbitrator agree (unusual case, possibly dispute)
        if job.dispute_flag {
            // If there's a dispute, arbitrator's decision prevails
            ctx.accounts.client.to_account_info()
        } else {
            // No dispute, default to worker
            ctx.accounts.worker_account.to_account_info()
        }
    } else {
        // Fallback (shouldn't happen due to approval check)
        return Err(ErrorCode::InsufficientApprovals.into());
    };

    // Transfer funds from escrow to recipient
    let transfer_amount = quote.proposed_budget;
    **escrow_account.try_borrow_mut_lamports()? -= transfer_amount;
    **recipient.try_borrow_mut_lamports()? += transfer_amount;

    // Update job state
    job.payment_released = true;  // Add this field to Job struct
    job.payment_released_at = Clock::get()?.unix_timestamp;  // Add this field to Job struct
    job.escrow_funded = false;  // Mark escrow as no longer funded

    // Close escrow account and reclaim rent if needed
    // Note: This can be done by transferring remaining lamports to a recipient
    // This is a safer approach than the previous system_program::transfer
    let remaining_balance = escrow_account.lamports();
    if remaining_balance > 0 {
        **escrow_account.try_borrow_mut_lamports()? = 0;
        **ctx.accounts.client.try_borrow_mut_lamports()? += remaining_balance;
    }
    Ok(())
}
