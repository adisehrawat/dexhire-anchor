use anchor_lang::prelude::*;

#[account]
pub struct ClientProfile {
    pub name: String,
    pub email: String,
    pub bio: String,
    pub country: String,
    pub linkedin: String,
    pub avatar: String,
    pub joined_at: i64,
    pub authority: Pubkey,
    pub bump: u8,
}
impl ClientProfile {
    pub const MAX_SIZE: usize = 4 + 64 + 4 + 64 + 4 + 256 + 4 + 64 + 4 + 64 + 8 + 32 + 1;
}