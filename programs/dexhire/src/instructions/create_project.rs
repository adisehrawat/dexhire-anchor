use anchor_lang::prelude::*;
use crate::states::{ Project, ClientProfile, ProjectCreated };
use crate::errors::ErrorCode;
pub fn create_project(
        ctx: Context<CreateProject>,
        name: String,
        about: String,
        price: u64,
        deadline: i64
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(deadline > now, ErrorCode::InvalidDeadline);

        let project = &mut ctx.accounts.project;
        project.set_inner(Project {
            name: name.clone(),
            about,
            creator: ctx.accounts.owner.key(),
            is_public: true,
            price,
            deadline,
            start: now,
            proposal: 0,
            freelancer: Pubkey::default(),
            github_link: None,
            work_submitted_at: None,
            is_completed: false,
            bump: ctx.bumps.project,
            vault_bump: ctx.bumps.vault,
        });

        emit!(ProjectCreated {
            project: project.key(),
            creator: ctx.accounts.owner.key(),
            name,
            price,
        });
        Ok(())
    }

#[derive(Accounts)]
#[instruction(name:String)]
pub struct CreateProject<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Project::MAX_SIZE,
        seeds = [b"project", name.as_bytes(), client.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub project: Account<'info, Project>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(seeds = [b"client", owner.key().as_ref()], bump = client.bump)]
    pub client: Account<'info, ClientProfile>,
    /// CHECK: vault PDA is created here and its lamports will be managed by the program
    #[account(init, payer = owner, seeds = [b"vault", project.key().as_ref()], bump, space = 8)]
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}