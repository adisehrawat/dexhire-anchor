use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::{ ProposalStatus,WorkApprovedAndPaid,Project,Proposal };

pub fn approve_work_and_pay(ctx: Context<ApproveWorkAndPay>) -> Result<()> {
        let project = &mut ctx.accounts.project;
        require!(project.creator == ctx.accounts.client.key(), ErrorCode::Unauthorized);
        require!(project.status()? == ProposalStatus::Accepted, ErrorCode::ProposalNotAccepted);
        require!(project.github_link.is_some(), ErrorCode::WorkNotSubmitted);

        // Transfer escrowed lamports from vault to freelancer
        let project_key = project.key();
        let vault_seeds = &[b"vault", project_key.as_ref(), &[project.vault_bump]];
        let _signer_seeds = &[&vault_seeds[..]];

        let vault_lamports = ctx.accounts.vault.lamports();
        require!(vault_lamports >= project.price, ErrorCode::VaultInsufficientFunds);

        **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= project.price;
        **ctx.accounts.freelancer.to_account_info().try_borrow_mut_lamports()? += project.price;

        project.is_completed = true;

        let proposal = &mut ctx.accounts.proposal;
        proposal.status = ProposalStatus::Completed;

        emit!(WorkApprovedAndPaid {
            project: project.key(),
            freelancer: ctx.accounts.freelancer.key(),
            amount: project.price,
        });
        Ok(())
    }

#[derive(Accounts)]
pub struct ApproveWorkAndPay<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    /// CHECK: not a PDA, just receives SOL
    #[account(mut)]
    pub freelancer: AccountInfo<'info>,
    #[account(mut)]
    pub client: Signer<'info>,
    /// CHECK: vault PDA seeds verified via project.vault_bump
    #[account(
        mut,
        seeds = [b"vault", project.key().as_ref()],
        bump = project.vault_bump,
    )]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}