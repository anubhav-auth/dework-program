use anchor_lang::prelude::*;
use crate::state::{job::*, quotes::*};

#[derive(Accounts)]
#[instruction(message: String)] // Ensure correct space allocation
pub struct SubmitQuote<'info>{
    #[account(
        init, 
        payer=worker, 
        space = 8 + 32 + 32 + 8 + 4 + message.capacity() + 1, // Adjust space dynamically
        seeds = [b"quote", job.key().as_ref(), worker.key().as_ref()], //helps with security unique PDA (Program Derived Address) so can t be duplicated
        bump
    )]
    pub quote: Account<'info, Quote>,

    #[account()]
    pub job: Account<'info, Job>,

    #[account(mut)]
    pub worker: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn submit_quote(
    ctx:Context<SubmitQuote>,
    proposed_budget:u64,
    message:String
) -> Result<()> {
    let quote = &mut ctx.accounts.quote;

    quote.worker = ctx.accounts.worker.key();
    quote.job = ctx.accounts.job.key();
    quote.proposed_budget = proposed_budget;
    quote.message = message;
    quote.accepted = false;
    

    Ok(())
}
