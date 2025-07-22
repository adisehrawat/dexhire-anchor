use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
#[allow(unused_imports)]
use states::*;

declare_id!("341BQ4r4HykJSTSr9XKWeR2fDt9d5WCSUCn4VS4q7iyg");

#[program]
pub mod dexhire {
    use super::*;

    pub fn approve_project(ctx: Context<ApproveProject>, _name: String) -> Result<()> {
        instructions::approve_project(ctx, _name)?;
        Ok(())
    }

    pub fn approve_work_and_pay(ctx: Context<ApproveWorkAndPay>) -> Result<()> {
        instructions::approve_work_and_pay(ctx)?;
        Ok(())
    }

    pub fn complete_project(ctx: Context<CompleteProject>, creator: Pubkey, _name: String) -> Result<()> {
        instructions::complete_project(ctx, creator, _name)?;
        Ok(())
    }

    pub fn create_client_profile(ctx: Context<CreateClientProfile>, name: String, email: String) -> Result<()> {
        instructions::create_client_profile(ctx, name, email)?;
        Ok(())
    }

    pub fn create_freelancer_profile(ctx: Context<CreateFreelancerProfile>, name: String, email: String) -> Result<()> {
        instructions::create_freelancer_profile(ctx, name, email)?;
        Ok(())
    }

    pub fn create_project(ctx: Context<CreateProject>, name: String, about: String, price: u64, deadline: i64) -> Result<()> {
        instructions::create_project(ctx, name, about, price, deadline)?;
        Ok(())
    }

    pub fn delete_client_profile(ctx: Context<DeleteClientProfile>) -> Result<()> {
        instructions::delete_client_profile(ctx)?;
        Ok(())
    }

    pub fn delete_freelancer_profile(_ctx: Context<DeleteFreelancerProfile>) -> Result<()> {
        instructions::delete_freelancer_profile(_ctx)?;
        Ok(())
    }

    pub fn fund_project(ctx: Context<FundProject>, lamports: u64) -> Result<()> {
        instructions::fund_project(ctx, lamports)?;
        Ok(())
    }

    pub fn respond_to_proposal(ctx: Context<RespondToProposal>, accept: bool) -> Result<()> {
        instructions::respond_to_proposal(ctx, accept)?;
        Ok(())
    }

    pub fn submit_proposal(ctx: Context<SubmitProposal>, _project_name: String, msg: String) -> Result<()> {
        instructions::submit_proposal(ctx, _project_name, msg)?;
        Ok(())
    }


    pub fn submit_work(ctx: Context<SubmitWork>, github_link: String) -> Result<()> {
        instructions::submit_work(ctx, github_link)?;
        Ok(())
    }


    pub fn update_client_profile(ctx: Context<UpdateClientProfile>, name: String, email: String, bio: String, country: String, linkedin: String, authority: Pubkey) -> Result<()> {
        instructions::update_client_profile(ctx, name, email, bio, country, linkedin, authority)?;
        Ok(())
    }

    pub fn update_freelancer_profile(ctx: Context<UpdateFreelancerProfile>, name: String, email: String, bio: String, country: String, linkedin: String, authority: Pubkey) -> Result<()> {
        instructions::update_freelancer_profile(ctx, name, email, bio, country, linkedin, authority)?;
        Ok(())
    }


}

