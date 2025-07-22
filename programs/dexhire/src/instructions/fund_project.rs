use anchor_lang::prelude::*;
use anchor_lang::system_program::{ transfer, Transfer };
use crate::states::{ Project };


pub fn fund_project(ctx: Context<FundProject>, lamports: u64) -> Result<()> {
        let project_key = ctx.accounts.project.key();
        let vault_seeds = &[b"vault", project_key.as_ref(), &[ctx.accounts.project.vault_bump]];
        let _signer_seeds = &[&vault_seeds[..]];

        transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), Transfer {
                from: ctx.accounts.client.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            }),
            lamports
        )?;
        Ok(())
    }

#[derive(Accounts)]
pub struct FundProject<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK: vault PDA seeds verified via project.vault_bump
    #[account(
        mut,
        seeds = [b"vault", project.key().as_ref()],
        bump = project.vault_bump,
    )]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}