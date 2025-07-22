

use anchor_lang::prelude::*;
use crate::states::{ClientProfile};

pub fn create_client_profile(ctx: Context<CreateClientProfile>, name: String, email: String) -> Result<()> {
        let profile = &mut ctx.accounts.client_profile;
        profile.set_inner(ClientProfile {
            name: name.clone(),
            email,
            bio: String::new(),
            country: String::new(),
            linkedin: String::new(),
            avatar: process_avtar(&name),
            joined_at: Clock::get()?.unix_timestamp,
            authority: ctx.accounts.owner.key(),
            bump: ctx.bumps.client_profile,
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
pub struct CreateClientProfile<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + ClientProfile::MAX_SIZE,
        seeds = [b"client", owner.key().as_ref()],
        bump
    )]
    pub client_profile: Account<'info, ClientProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}