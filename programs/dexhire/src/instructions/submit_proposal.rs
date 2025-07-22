use anchor_lang::prelude::*;
use crate::states::{ Proposal,ProposalStatus,ProposalSubmitted,FreelancerProfile,Project };

pub fn submit_proposal(ctx: Context<SubmitProposal>, _project_name: String, msg: String) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        let proposal = &mut ctx.accounts.proposal;
        proposal.set_inner(Proposal {
            freelancer: ctx.accounts.freelancer.key(),
            project: ctx.accounts.project.key(),
            client: ctx.accounts.project.creator,
            msg,
            msg_time: now,
            status: ProposalStatus::Pending,
            bump: ctx.bumps.proposal,
        });

        ctx.accounts.project.proposal += 1;

        emit!(ProposalSubmitted {
            project: ctx.accounts.project.key(),
            freelancer: ctx.accounts.freelancer.key(),
        });
        Ok(())
    }

#[derive(Accounts)]
#[instruction(project_name:String)]
pub struct SubmitProposal<'info> {
    #[account(
        init,
        payer = freelancer_signer,
        space = 8 + Proposal::MAX_SIZE,
        seeds = [b"proposal", project.key().as_ref(), freelancer.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub freelancer_signer: Signer<'info>,
    #[account(seeds = [b"freelancer", freelancer_signer.key().as_ref()], bump = freelancer.bump)]
    pub freelancer: Account<'info, FreelancerProfile>,
    #[account(mut)]
    pub project: Account<'info, Project>,
    /// CHECK: only used as key
    pub client: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}