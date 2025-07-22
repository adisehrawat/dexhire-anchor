

use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile};

pub fn create_freelancer_profile(ctx: Context<CreateFreelancerProfile>, name: String, email: String) -> Result<()> {
        let profile = &mut ctx.accounts.freelancer_profile;
        profile.set_inner(FreelancerProfile {
            name: name.clone(),
            email,
            bio: String::new(),
            country: String::new(),
            linkedin: String::new(),
            avatar: process_avtar(&name),
            joined_at: Clock::get()?.unix_timestamp,
            authority: ctx.accounts.owner.key(),
            bump: ctx.bumps.freelancer_profile,
        });
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
pub struct CreateFreelancerProfile<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + FreelancerProfile::MAX_SIZE,
        seeds = [b"freelancer", owner.key().as_ref()],
        bump
    )]
    pub freelancer_profile: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}