use anchor_lang::prelude::*;
use crate::states::{ProposalStatus};
#[account]
pub struct Project {
    pub name: String,
    pub about: String,
    pub creator: Pubkey,
    pub is_public: bool,
    pub price: u64,
    pub deadline: i64,
    pub start: i64,
    pub proposal: u64,
    pub freelancer: Pubkey,
    pub github_link: Option<String>,
    pub work_submitted_at: Option<i64>,
    pub is_completed: bool,
    pub bump: u8,
    pub vault_bump: u8,
}
impl Project {
    pub const MAX_SIZE: usize = 4 + 64 + 4 + 512 + 32 + 1 + 8 + 8 + 8 + 8 + 32 + (1 + 4 + 128) + (1 + 8) + 1 + 1 + 1;
}

impl Project {
    pub fn status(&self) -> Result<ProposalStatus> {
        if self.freelancer == Pubkey::default() {
            return Ok(ProposalStatus::Pending);
        }
        // simplistic: if freelancer set then proposal accepted
        Ok(ProposalStatus::Accepted)
    }
}