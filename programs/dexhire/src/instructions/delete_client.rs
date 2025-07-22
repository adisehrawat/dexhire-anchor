use anchor_lang::prelude::*;
use crate::states::{ClientProfile};

pub fn delete_client_profile(_ctx: Context<DeleteClientProfile>) -> Result<()> {
        Ok(())
    }

#[derive(Accounts)]
pub struct DeleteClientProfile<'info> {
    #[account(mut, close = owner)]
    pub client_profile: Account<'info, ClientProfile>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}