#![allow(unexpected_cfgs)]


pub mod instructions;
pub mod state;
pub use instructions::{create_job::*, update_job::*, delete_job::*, submit_quote::*};



use anchor_lang::prelude::*;

declare_id!("92sorgqaDHqG5T12ZqrTMSCFZEHaVxrANLWdSQ5fFUom");

#[program]
pub mod dework_program {
    use super::*;

    pub fn create_new_job(
        ctx: Context<CreateJob>, 
        title: String, 
        description: String, 
        budget: u64
    ) -> Result<()> {
        create_job(ctx, title, description, budget)
    }

    pub fn update_existing_job(
        ctx: Context<UpdateJob>, 
        title: Option<String>, 
        description: Option<String>, 
        budget: Option<u64>, 
        is_open: Option<bool>,
        job_complete: Option<bool>
    ) -> Result<()> {
        update_job(ctx, title, description, budget, is_open, job_complete)
    }

    pub fn delete_existing_job(
        ctx: Context<DeleteJob>
    ) -> Result<()> {
        delete_job(ctx)
    }

    pub fn submit_new_quote(
        ctx: Context<SubmitQuote>, 
        proposed_budget: u64,
        message: String
    ) -> Result<()>{

        submit_quote(ctx, proposed_budget, message)
    }

}