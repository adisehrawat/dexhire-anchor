use anchor_lang::prelude::*;
use crate::states::{FreelancerProfile};

pub fn delete_freelance_profile(ctx: Context<DeleteFreelanceProfile>) -> Result<()>{
    let _freelanceprofile = &mut ctx.accounts.freelanceprofile;
    msg!("profile deleted");
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteFreelanceProfile<'info>{
    #[account(mut,
         seeds = [ b"freelancer",owner.key().as_ref()],
         bump,
         close = owner,
        )]
    pub freelanceprofile: Account<'info, FreelancerProfile>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}