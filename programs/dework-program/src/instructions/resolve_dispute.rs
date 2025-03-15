use anchor_lang::{prelude::*, system_program};
use crate::state::{job::*, quotes::*};

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    
    #[msg("Job is not under dispute")]
    JobNotInDispute,
    
    #[msg("Invalid resolution option")]
    InvalidResolutionOption,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info>{

    #[account(mut, has_one = arbitrator @ ErrorCode::Unauthorized)]
    pub job: Account<'info, Job>,

    #[account()]
    pub quote: Account<'info, Quote>,

    #[account(mut, seeds = [b"escrow", job.key().as_ref(), quote.worker.key().as_ref()], bump)]
    pub escrow_account: SystemAccount<'info>,
    
    #[account(mut, address = job.client)]
    pub client_account: SystemAccount<'info>,
    
    #[account(mut, address = quote.worker)]
    pub worker_account: SystemAccount<'info>,
    
    pub arbitrator: Signer<'info>,
    
    pub system_program: Program<'info, System>,

}

pub fn resolve_dispute(ctx: Context<ResolveDispute>, resolution: u8, split: Option<u8>) -> Result<()> {
    let job = &mut ctx.accounts.job;
    let quote = &ctx.accounts.quote;
    let escrow_account = &mut ctx.accounts.escrow_account;
    let client_account = &ctx.accounts.client_account;
    let worker_account = &ctx.accounts.worker_account;
    let amount = quote.proposed_budget;

    require!(job.dispute_flag, ErrorCode::JobNotInDispute);
    
    let mut recipient = worker_account.to_account_info();
    let mut payout_amount = amount;

    match resolution {
        0 => { 
            recipient = client_account.to_account_info();
            payout_amount = amount;
        }
        1 => { 
            recipient = worker_account.to_account_info();
            payout_amount = amount;
        }
        2 => { 
            let split_ratio = split.unwrap_or(50) as u64; 
            require!(split_ratio <= 100, ErrorCode::InvalidResolutionOption);

            payout_amount = (split_ratio * amount) / 100; 
            
            // Transfer remaining amount back to client
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: escrow_account.to_account_info(),
                        to: client_account.to_account_info(),
                    },
                ),
                amount - payout_amount,
            )?;
        }
        _ => return Err(ErrorCode::InvalidResolutionOption.into()),
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: escrow_account.to_account_info(),
                to: recipient,
            },
        ),
        payout_amount,
    )?;

    Ok(())
}





