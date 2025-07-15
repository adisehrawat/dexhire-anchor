use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FreelancerProfile {
    #[max_len(64)]
    pub name: String,
    #[max_len(64)]
    pub email: String,
    #[max_len(512)]
    pub bio: String,
    #[max_len(20)]
    pub skills: Vec<Skill>,
    #[max_len(64)]
    pub country: String,
    #[max_len(64)]
    pub linkedin: String,
    #[max_len(64)]
    pub avatar: String,
    pub joined_at: i64,
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Skill {
    #[max_len(32)]
    pub name: String,
}