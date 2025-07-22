use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile};
use crate::errors::ErrorCode;

pub fn update_freelancer_profile(
        ctx: Context<UpdateFreelancerProfile>,
        name: String,
        email: String,
        bio: String,
        country: String,
        linkedin: String,
        authority: Pubkey
    ) -> Result<()> {
        require!(ctx.accounts.freelancer_profile.authority == authority, ErrorCode::WrongAuthority);
        let profile = &mut ctx.accounts.freelancer_profile;
        profile.name = name.clone();
        profile.email = email;
        profile.bio = bio;
        profile.country = country;
        profile.linkedin = linkedin;
        profile.avatar = process_avtar(&name);
        Ok(())
    }

fn process_avtar(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.contains(' ') {
        trimmed
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .collect()
    } else {
        trimmed.chars().take(2).collect()
    }
}

#[derive(Accounts)]
pub struct UpdateFreelancerProfile<'info> {
    #[account(mut)]
    pub freelancer_profile: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}