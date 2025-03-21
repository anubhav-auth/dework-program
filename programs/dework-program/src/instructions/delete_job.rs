use anchor_lang::prelude::*;
use crate::state::job::*;
use crate::instructions::errors::ErrorCode;


#[derive(Accounts)]
pub struct DeleteJob<'info>{
    #[account(mut, has_one = client @ ErrorCode::Unauthorized)]
    pub job: Account<'info,Job>,
    pub client: Signer<'info>,
    pub system_program: Program<'info, System>
}


pub fn delete_job(ctx:Context<DeleteJob>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let client = &mut ctx.accounts.client;

    require!(job.is_accepted, ErrorCode::JobAcceptedAlready);
    
    let job_lamports = job.to_account_info().lamports();

    **client.to_account_info().try_borrow_mut_lamports()? += job_lamports;
    **job.to_account_info().try_borrow_mut_lamports()? = 0;

    let account_info = job.to_account_info();
    let mut data = account_info.try_borrow_mut_data()?;
    data.fill(0);

    Ok(())
}