use anchor_lang::prelude::*;
use crate::state::job::*;
use crate::instructions::errors::ErrorCode;


#[derive(Accounts)]
pub struct UpdateJob<'info>{

    #[account(mut, has_one = client @ ErrorCode::Unauthorized )]
    pub job: Account<'info, Job>,
    pub client: Signer<'info>,
    pub system_program: Program<'info, System>

}

pub fn update_job(
    ctx: Context<UpdateJob>,
    title: Option<String>,
    description: Option<String>,
    budget: Option<u64>,
    is_open: Option<bool>,
    job_complete: Option<bool>
) -> Result<()>{
    let job = &mut ctx.accounts.job;

    if let Some(new_title) = title{
        job.title = new_title;
    }
    if let Some(new_description) = description {
        job.description = new_description;
    }
    
    if let Some(new_budget) = budget {
        job.budget = new_budget;
    }
    
    if let Some(new_is_open) = is_open {

        if job.job_completed {
            return Err(ErrorCode::MarkedComplete.into());
        }

        job.is_open = new_is_open;
    }

    if let Some(new_job_complete) = job_complete {

        if job.job_completed {
            return Err(ErrorCode::MarkedComplete.into());
        }

        job.job_completed = new_job_complete;
    }

    Ok(())
}