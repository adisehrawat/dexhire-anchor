use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ClientProfile {
    #[max_len(64)]
    pub name: String,
    #[max_len(64)]
    pub email: String,
    #[max_len(512)]
    pub bio: String,
    #[max_len(64)]
    pub country: String,
    #[max_len(64)]
    pub linkedin: String,
    #[max_len(64)]
    pub avatar: String,
    pub joined_at: i64,
    pub authority: Pubkey,
}
