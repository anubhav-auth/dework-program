use anchor_lang::prelude::*;
use crate::state::job::*;

// Updated space calculation for Job account
#[derive(Accounts)]
pub struct CreateJob<'info> {
    #[account(
        init, 
        payer = client, 
        space = 8 +                  // Discriminator
               32 +                  // client: Pubkey
               32 +                  // worker: Pubkey
               32 +                  // arbitrator: Pubkey
               4 + 50 +              // title: String (max 50 chars)
               4 + 500 +             // description: String (max 500 chars)
               8 +                   // budget: u64
               1 +                   // is_open: bool
               1 +                   // job_completed: bool
               1 +                   // dispute_flag: bool
               1 +                   // client_signed: bool
               1 +                   // worker_signed: bool
               1 +                   // arbitrator_signed: bool
               1 +                   // signatures: u8
               1 + 1 +               // arbitrator_decision: Option<u8>
               1 +                   // escrow_funded: bool
               8 +                   // escrow_amount: u64
               1 +                   // dispute_resolved: bool
               8 +                   // dispute_resolved_at: i64
               1 +                   // payment_released: bool
               8 +                    // payment_released_at: i64
               1 +                    // dispute_raised_by_client :bool
               1                     // dispute_raised_by_worker :bool
    )]
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
    job.job_completed = false;
    job.dispute_flag = false;
    
    // Initialize all other fields to prevent unexpected behavior
    job.worker = Pubkey::default();
    job.arbitrator = Pubkey::default();
    job.client_signed = false;
    job.worker_signed = false;
    job.arbitrator_signed = false;
    job.signatures = 0;
    job.arbitrator_decision = None;
    job.escrow_funded = false;
    job.escrow_amount = 0;
    job.dispute_resolved = false;
    job.dispute_resolved_at = 0;
    job.payment_released = false;
    job.payment_released_at = 0;
    
    Ok(())
}