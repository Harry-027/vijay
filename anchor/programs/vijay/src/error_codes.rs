use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow occurred during math operation")]
    NumericalOverflow,
    #[msg("Owner has already been initialized")]
    AlreadyInitialized,
    #[msg("Amount overflow occurred during math operation")]
    AmountOverflow,
    #[msg("Only project owner allowed to setup the escrow")]
    UnAuthorizedSetup,
    #[msg("Only owner allowed this operation")]
    UnAuthorizedOwner,
    #[msg("Only project owner allowed to review the project")]
    UnAuthorizedReviewer,
    #[msg("Escrow account is inactive")]
    EscrowInActive,
    #[msg("Project is inactive")]
    ProjectInActive,
    #[msg("Freelancer project is inactive")]
    FreelancerProjectInActive,
    #[msg("All the tasks have been completed")]
    TasksCompleted,
    #[msg("Only owner is allowed to proceed with this operation")]
    NotAnOwner,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Task review not yet requested")]
    TaskReviewNotRequested,
    #[msg("Bad request::Too long")]
    TooLong
}
