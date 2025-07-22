use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile};

pub fn delete_freelancer_profile(_ctx: Context<DeleteFreelancerProfile>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteFreelancerProfile<'info> {
    #[account(mut, close = owner)]
    pub freelancer_profile: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}