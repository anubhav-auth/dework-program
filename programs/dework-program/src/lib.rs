use anchor_lang::prelude::*;

declare_id!("92sorgqaDHqG5T12ZqrTMSCFZEHaVxrANLWdSQ5fFUom");

#[program]
pub mod dework_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[account]
pub struct Job{
    pub client: Pubkey, // key of client stored here
    pub title: String, // title of job
    pub description: String, // description of job
    pub budget: String, // //budget allocated for the job
    pub is_open: bool // wether the job is still available or not
}