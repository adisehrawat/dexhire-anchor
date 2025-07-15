use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile, Skill};
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn update_freelance_profile(ctx: Context<UpdateFreelanceProfile>,name: String,email: String,bio:String,skills: Vec<Skill>,country: String,linkedin: String) -> Result<()> {
    let freelancerprofile = &mut ctx.accounts.freelanceprofile;
    freelancerprofile.name = name.clone();
    freelancerprofile.email = email;
    freelancerprofile.bio = bio;
    freelancerprofile.skills = skills;
    freelancerprofile.country = country;
    freelancerprofile.linkedin = linkedin;
    freelancerprofile.avatar = process_avtar(&name);
    msg!("profile updated");
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
pub struct UpdateFreelanceProfile<'info>{
    #[account(
        mut,
        seeds = [ b"freelancer",owner.key().as_ref()],
        bump,
        realloc = ANCHOR_DISCRIMINATOR_SIZE + FreelancerProfile::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub freelanceprofile: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

}