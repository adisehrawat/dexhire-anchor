

use anchor_lang::prelude::*;
use crate::states::{ClientProfile};
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn create_client_profile(ctx: Context<CreateClientProfile>,name: String,email: String,) -> Result<()> {
    let clientprofile = &mut ctx.accounts.clientprofile;
    clientprofile.name = name.clone();
    clientprofile.email = email;
    clientprofile.joined_at = Clock::get()?.unix_timestamp;
    clientprofile.authority = ctx.accounts.owner.key();
    clientprofile.country = String::from("");
    clientprofile.bio = String::from("");
    clientprofile.linkedin = String::from("");
    clientprofile.avatar = process_avtar(&name);
    msg!("client account created");
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
pub struct CreateClientProfile<'info>{
    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + ClientProfile::INIT_SPACE,
        seeds = [ b"client",owner.key().as_ref()],
        bump,
    )]
    pub clientprofile: Account<'info, ClientProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

}