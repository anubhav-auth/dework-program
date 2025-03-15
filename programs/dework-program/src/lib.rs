#![allow(unexpected_cfgs)]


pub mod instructions;
pub mod state;
pub use instructions::create_job::*;


use anchor_lang::prelude::*;

declare_id!("92sorgqaDHqG5T12ZqrTMSCFZEHaVxrANLWdSQ5fFUom");

#[program]
pub mod dework_program {
    use super::*;

    pub fn create_new_job(ctx: Context<CreateJob>, title: String, description: String, budget: u64) -> Result<()> {
        create_job(ctx, title, description, budget)
    }
}



