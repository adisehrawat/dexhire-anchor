use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::{ Project,ClientProfile };

pub fn complete_project(ctx: Context<CompleteProject>, creator: Pubkey, _name: String) -> Result<()> {
        require!(ctx.accounts.project.creator == creator, ErrorCode::InvalidCreator);
        ctx.accounts.project.is_completed = true;
        Ok(())
    }

#[derive(Accounts)]
#[instruction(creator:Pubkey, name:String)]
pub struct CompleteProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    pub owner: Signer<'info>,
    #[account(seeds = [b"client", owner.key().as_ref()], bump = client.bump)]
    pub client: Account<'info, ClientProfile>,
    pub system_program: Program<'info, System>,
}