

use anchor_lang::prelude::*;
use crate::states::{ClientProfile};
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn update_client_profile(ctx: Context<UpdateClientProfile>,name: String,email: String,bio:String,country: String,linkedin: String) -> Result<()> {
    let clientprofile = &mut ctx.accounts.clientprofile;
    clientprofile.name = name.clone();
    clientprofile.email = email;
    clientprofile.bio = bio;
    clientprofile.country = country;
    clientprofile.linkedin = linkedin;
    clientprofile.avatar = process_avtar(&name);
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
pub struct UpdateClientProfile<'info>{
    #[account(
        mut,
        seeds = [ b"client",owner.key().as_ref()],
        bump,
        realloc = ANCHOR_DISCRIMINATOR_SIZE + ClientProfile::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub clientprofile: Account<'info, ClientProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

}