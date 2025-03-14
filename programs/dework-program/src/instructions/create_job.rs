// src/instructions.rs
use anchor_lang::prelude::*;
use crate::state::job::*;

#[derive(Accounts)]
pub struct CreateJob<'info> {
    #[account(init, payer = client, space = 8 + 32 + 100 + 500 + 8 + 1)]
    pub job: Account<'info, Job>,  
    #[account(mut)]
    pub client: Signer<'info>,  
    pub system_program: Program<'info, System>,  
}

pub fn create_job(
    ctx: Context<CreateJob>,
    title: String,
    description: String,
    budget: u64,
) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.client = *ctx.accounts.client.key;
    job.title = title;
    job.description = description;
    job.budget = budget;
    job.is_open = true;
    Ok(())
}