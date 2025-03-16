use anchor_lang::prelude::*;
use crate::state::{job::*, quotes::*};

#[derive(Accounts)]
#[instruction(message: String)]
pub struct SubmitQuote<'info>{
    #[account(
        init, 
        payer = worker, 
        space = 8 +                  // Discriminator
               32 +                  // client: Pubkey
               32 +                  // worker: Pubkey
               32 +                  // job: Pubkey
               8 +                   // proposed_budget: u64
               4 + message.len() +   // message: String (dynamic)
               1 +                   // accepted: bool
               8 +                   // accepted_at: i64
               1 + 1 +               // dispute_resolution: Option<u8>
               10                    // Buffer for potential additional space needs
    )]
    pub quote: Account<'info, Quote>,

    #[account()]
    pub job: Account<'info, Job>,

    #[account(mut)]
    pub worker: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn submit_quote(
    ctx: Context<SubmitQuote>,
    proposed_budget: u64,
    message: String
) -> Result<()> {
    let quote = &mut ctx.accounts.quote;

    quote.worker = ctx.accounts.worker.key();
    quote.client = ctx.accounts.job.client;
    quote.job = ctx.accounts.job.key();
    quote.proposed_budget = proposed_budget;
    quote.message = message;
    quote.accepted = false;
    quote.accepted_at = 0;
    quote.dispute_resolution = None;
    
    Ok(())
}
