use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the owner can perform this action")]
    Unauthorized,
    #[msg("Wrong authority")]
    WrongAuthority,
    #[msg("The creator is not authorized to create this project")]
    InvalidCreator,
    #[msg("The recipient public key is invalid")]
    InvalidRecipient,
    #[msg("Wrong project")]
    WrongProject,
    #[msg("Proposal has already been responded to")]
    ProposalAlreadyResponded,
    #[msg("Proposal is not accepted")]
    ProposalNotAccepted,
    #[msg("Work has not been submitted (missing github link)")]
    WorkNotSubmitted,
    #[msg("Vault does not have enough funds to pay freelancer")]
    VaultInsufficientFunds,
    #[msg("Invalid deadline provided")]
    InvalidDeadline,
}