use std::thread::AccessError;

use anchor_lang::{prelude::*, solana_program::system_instruction::transfer, system_program::Transfer};
use crate::state::{job::*, quotes::*};


#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Quote does not belong to this job")]
    InvalidQuote,

    #[msg("A quote has already been accepted for this job")]
    QuoteAlreadyAccepted,
}

#[derive(Accounts)]
pub struct AcceptQuote<'info>{
    
    #[account(mut, has_one = client @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account(mut, has_one = job @ ErrorCode::InvalidQuote)]
    pub quote: Account<'info, Quote>,

    pub client:Signer<'info>,

    #[account(mut, seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()], bump)]
    pub escrow_account: UncheckedAccount<'info>, 

    pub system_program: Program<'info, System>
}


pub fn accept_quote(ctx:Context<AcceptQuote>)-> Result<()>{
    let job = &mut ctx.accounts.job;
    let quote = &mut ctx.accounts.quote;
    let escrow_account = &mut ctx.accounts.escrow_account.to_account_info();
    let client = &mut ctx.accounts.client.to_account_info();
    let sys

    if !job.is_open {
        return Err(ErrorCode::QuoteAlreadyAccepted.into());
    }

    let transfer_ix = Transfer{
        from: client.clone(),
        to: escrow_account.clone()
    };

    // Execute the transfer instruction
    invoke(
        &transfer_ix,
        &[
            client.to_account_info(),
            escrow_account.to_account_info(),
            system_program.to_account_info(),
        ],
    )?;


    job.is_open = false;
    quote.accepted = true;

    Ok(())
}