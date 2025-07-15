use anchor_lang::prelude::*;
use crate::states::{ClientProfile};

pub fn delete_client_profile(ctx: Context<DeleteClientProfile>) -> Result<()> {
    let _clientprofile = &mut ctx.accounts.clientprofile;
    msg!("profile deleted");
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteClientProfile<'info>{
    #[account(mut,
         seeds = [ b"client",owner.key().as_ref()],
         bump,
         close = owner,
        )]
    pub clientprofile: Account<'info, ClientProfile>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}