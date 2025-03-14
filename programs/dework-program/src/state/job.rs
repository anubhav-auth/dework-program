use anchor_lang::prelude::*;

#[account]
pub struct Job{
    pub client: Pubkey, // key of client stored here
    pub title: String, // title of job
    pub description: String, // description of job
    pub budget: u64, // //budget allocated for the job
    pub is_open: bool // wether the job is still available or not
}