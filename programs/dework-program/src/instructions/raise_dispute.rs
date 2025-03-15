use anchor_lang::prelude::*;
use crate::state::job::*;

#[error_code]
pub enum ErrorCode {
    #[msg("")]
    JobClosed,

    #[msg("")]
    DisputeAlreadyRaised
}


#[derive(Accounts)]
pub struct RaiseDispute<'info> {
    #[account(mut, has_one = client, has_one = worker)]
    pub job: Account<'info, Job>,
    pub client: Signer<'info>,
    pub worker: Signer<'info>,
}

pub fn raise_dispute(ctx: Context<RaiseDispute>) -> Result<()> {
    let job = &mut ctx.accounts.job;

    require!(job.is_open, ErrorCode::JobClosed);
    require!(!job.dispute_flag, ErrorCode::DisputeAlreadyRaised);

    job.dispute_flag = true;

    Ok(())
}

