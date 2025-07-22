use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::{ Project,ClientProfile };

pub fn approve_project(ctx: Context<ApproveProject>, _name: String) -> Result<()> {
        // Only owner of the client profile can approve
        require!(ctx.accounts.client.authority == ctx.accounts.owner.key(), ErrorCode::Unauthorized);
        // We just flip is_public (or any other approval semantics)
        ctx.accounts.project.is_public = true;
        Ok(())
    }

#[derive(Accounts)]
#[instruction(name:String)]
pub struct ApproveProject<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    pub owner: Signer<'info>,
    #[account(seeds = [b"client", owner.key().as_ref()], bump = client.bump)]
    pub client: Account<'info, ClientProfile>,
    pub system_program: Program<'info, System>,
}