use anchor_lang::prelude::*;

#[account]
pub struct Quote{
    pub client: Pubkey,
    pub worker: Pubkey,
    pub job: Pubkey,
    pub proposed_budget: u64,
    pub message: String,
    pub accepted: bool
}