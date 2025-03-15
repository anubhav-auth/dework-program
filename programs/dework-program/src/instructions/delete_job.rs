use anchor_lang::{prelude::*, solana_program::bpf_loader_upgradeable::close};
use crate::state::job::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
}


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
    
    let job_lamports = job.to_account_info().lamports();

    **job.to_account_info().try_borrow_mut_lamports()? = 0;
    **client.to_account_info().try_borrow_mut_lamports()? += job_lamports;


    let account_info = job.to_account_info();
    let mut data = account_info.try_borrow_mut_data()?;

    for byte in data.iter_mut() {
        *byte = 0;
    }
    Ok(())
}