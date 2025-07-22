use anchor_lang::prelude::*;

#[event]
pub struct ProjectCreated {
    pub project: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub price: u64,
}

#[event]
pub struct ProposalSubmitted {
    pub project: Pubkey,
    pub freelancer: Pubkey,
}

#[event]
pub struct ProposalAccepted {
    pub project: Pubkey,
    pub freelancer: Pubkey,
}

#[event]
pub struct ProposalRejected {
    pub project: Pubkey,
    pub freelancer: Pubkey,
}

#[event]
pub struct WorkSubmitted {
    pub project: Pubkey,
    pub github_link: String,
}

#[event]
pub struct WorkApprovedAndPaid {
    pub project: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
}