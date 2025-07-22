use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::{ ProposalStatus,ProposalAccepted,ProposalRejected,Proposal,Project,ClientProfile };


pub fn respond_to_proposal(ctx: Context<RespondToProposal>, accept: bool) -> Result<()> {
        require!(ctx.accounts.project.creator == ctx.accounts.client_signer.key(), ErrorCode::Unauthorized);
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.status == ProposalStatus::Pending, ErrorCode::ProposalAlreadyResponded);

        proposal.status = if accept { ProposalStatus::Accepted } else { ProposalStatus::Rejected };

        if accept {
            ctx.accounts.project.freelancer = proposal.freelancer;
            emit!(ProposalAccepted {
                project: ctx.accounts.project.key(),
                freelancer: proposal.freelancer,
            });
        } else {
            emit!(ProposalRejected {
                project: ctx.accounts.project.key(),
                freelancer: proposal.freelancer,
            });
        }
        Ok(())
    }

#[derive(Accounts)]
pub struct RespondToProposal<'info> {
    #[account(
        mut,
        seeds = [b"proposal", project.key().as_ref(), freelancer.key().as_ref()],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub client_signer: Signer<'info>,

    // 👇  MUST match create_project seeds
    #[account(
        mut,
        seeds = [
            b"project",
            project.name.as_bytes(),
            client_profile.key().as_ref(),   // <- client profile PDA
            client_signer.key().as_ref(),    // <- actual client signer
        ],
        bump = project.bump,
    )]
    pub project: Account<'info, Project>,

    /// CHECK: not a PDA
    #[account(mut)]
    pub freelancer: AccountInfo<'info>,

    // 🆕  add the client-profile PDA so we can include it in seeds
    #[account(
        seeds = [b"client", client_signer.key().as_ref()],
        bump = client_profile.bump,
    )]
    pub client_profile: Account<'info, ClientProfile>,
}