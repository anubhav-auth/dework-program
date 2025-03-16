use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Quote does not belong to this job")]
    InvalidQuote,

    #[msg("A quote has already been accepted for this job")]
    QuoteAlreadyAccepted,

    #[msg("The job is closed and no longer accepting proposals")]
    JobClosed,

    #[msg("A dispute has already been raised for this job")]
    DisputeAlreadyRaised,

    #[msg("Job is not marked as complete")]
    InvalidPaymentRequest,

    #[msg("At least 2 approvals required to release payment")]
    InsufficientApprovals,
    
    #[msg("Job is not under dispute")]
    JobNotInDispute,
    
    #[msg("Invalid resolution option")]
    InvalidResolutionOption,

    #[msg("Job is already marked as complete")]
    MarkedComplete,

    #[msg("Invalid quote ammount")]
    InvalidQuoteAmount,

    #[msg("Insufficient funds in escrow")]
    InsufficientFunds
}