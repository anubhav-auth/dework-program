use anchor_lang::prelude::*;
use crate::state::job::*;
use crate::instructions::errors::ErrorCode;


#[derive(Accounts)]
pub struct RaiseDispute<'info> {
    #[account(mut)]
    pub job: Account<'info, Job>,
    pub signer: Signer<'info>,
}

pub fn raise_dispute(ctx: Context<RaiseDispute>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let signer_key = ctx.accounts.signer.key();

    // Verify the signer is either the client or worker
    require!(
        signer_key == job.client || signer_key == job.worker,
        ErrorCode::Unauthorized
    );
    
    require!(job.is_open, ErrorCode::JobClosed);
    require!(!job.dispute_flag, ErrorCode::DisputeAlreadyRaised);

    job.dispute_flag = true;
    
    // Optionally track which party raised the dispute
    if signer_key == job.client {
        job.dispute_raised_by_client = true;
    } else {
        job.dispute_raised_by_worker = true;
    }

    Ok(())
}