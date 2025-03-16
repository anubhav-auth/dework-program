use crate::instructions::errors::ErrorCode;
use crate::state::{job::*, quotes::*};
use anchor_lang::{prelude::*, system_program};

#[derive(Accounts)]
pub struct AcceptQuote<'info> {
    #[account(mut, has_one = client @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account(mut, has_one = job @ ErrorCode::InvalidQuote)]
    pub quote: Account<'info, Quote>,

    #[account(mut)]
    pub client: Signer<'info>,

    /// CHECK: This is a PDA derived from the job and worker keys, used as an escrow account
    #[account(
        mut,
        seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()],
        bump,
    )]
    pub escrow_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn accept_quote(ctx: Context<AcceptQuote>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let quote = &mut ctx.accounts.quote;

    if !job.is_open {
        return Err(ErrorCode::QuoteAlreadyAccepted.into());
    }

    require!(quote.proposed_budget > 0, ErrorCode::InvalidQuoteAmount);

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.client.to_account_info(),
                to: ctx.accounts.escrow_account.to_account_info(),
            },
        ),
        quote.proposed_budget,
    )?;

    job.is_open = false;
    job.worker = quote.worker;
    job.escrow_funded = true;  
    job.escrow_amount = quote.proposed_budget;
    quote.accepted = true;
    quote.accepted_at = Clock::get()?.unix_timestamp;


    Ok(())
}
