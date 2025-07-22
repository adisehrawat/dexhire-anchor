use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::{ ProposalStatus,WorkSubmitted,Project };

pub fn submit_work(ctx: Context<SubmitWork>, github_link: String) -> Result<()> {
        let project = &mut ctx.accounts.project;
        msg!("project.freelancer  = {}", project.freelancer);
        msg!("signer              = {}", ctx.accounts.freelancer.key());
        require!(
            project.freelancer == ctx.accounts.freelancer.key(), // ← compare *signer*
            ErrorCode::Unauthorized
        );
        require!(project.status()? == ProposalStatus::Accepted, ErrorCode::ProposalNotAccepted);

        project.github_link = Some(github_link.clone());
        project.work_submitted_at = Some(Clock::get()?.unix_timestamp);

        emit!(WorkSubmitted {
            project: project.key(),
            github_link,
        });
        Ok(())
    }

#[derive(Accounts)]
pub struct SubmitWork<'info> {
    #[account(mut)]
    pub project: Account<'info, Project>,
    pub freelancer: Signer<'info>,
}