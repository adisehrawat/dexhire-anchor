

use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile};
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn create_freelance_profile(ctx: Context<CreateFreelanceProfile>,name: String,email: String,) -> Result<()> {
    let freelancerprofile = &mut ctx.accounts.freelanceprofile;
    freelancerprofile.name = name.clone();
    freelancerprofile.email = email;
    freelancerprofile.joined_at = Clock::get()?.unix_timestamp;
    freelancerprofile.authority = ctx.accounts.owner.key();
    freelancerprofile.country = String::from("");
    freelancerprofile.bio = String::from("");
    freelancerprofile.skills = Vec::new();
    freelancerprofile.linkedin = String::from("");
    freelancerprofile.avatar = process_avtar(&name);
    msg!("freelance account created");
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
pub struct CreateFreelanceProfile<'info>{
    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + FreelancerProfile::INIT_SPACE,
        seeds = [ b"freelancer",owner.key().as_ref()],
        bump,
    )]
    pub freelanceprofile: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

}