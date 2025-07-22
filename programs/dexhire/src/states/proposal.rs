use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub freelancer: Pubkey,
    pub project: Pubkey,
    pub client: Pubkey,
    pub msg: String,
    pub msg_time: i64,
    pub status: ProposalStatus,
    pub bump: u8,
}
impl Proposal {
    pub const MAX_SIZE: usize = 32 + 32 + 32 + 4 + 512 + 8 + 1 + 1;
    pub fn real_status(&self) -> ProposalStatus {
        self.status.clone()
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
    Completed,
}