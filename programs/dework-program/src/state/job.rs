use anchor_lang::prelude::*;

#[account]
pub struct Job{
    pub client: Pubkey, // key of client stored here
    pub worker: Pubkey,        // Assigned worker
    pub arbitrator: Pubkey,    // Trusted arbitrator
    pub title: String, // title of job
    pub description: String, // description of job
    pub budget: u64, // //budget allocated for the job
    pub is_open: bool, // wether the job is still available or not
    pub job_completed: bool,
    pub dispute_flag: bool,    // True if dispute is raised
    pub client_signed: bool,   // Did the client approve release?
    pub worker_signed: bool,   // Did the worker approve release?
    pub arbitrator_signed: bool, // Did the arbitrator step in?
    pub signatures: u8,     // Total approvals (2 needed for payout)
    pub arbitrator_decision: Option<u8>,
    pub escrow_funded: bool,
    pub escrow_amount: u64,
    pub dispute_resolved: bool,
    pub dispute_resolved_at: i64,
    pub payment_released: bool,
    pub payment_released_at: i64,
    pub dispute_raised_by_worker: bool,
    pub dispute_raised_by_client: bool,
}