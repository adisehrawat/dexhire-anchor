use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
#[allow(unused_imports)]
use states::*;

declare_id!("53AaBgyUc8W2DbT2qEHYVdL2z7c9CGabF139WAeiTRKz");

#[program]
pub mod dexhire {
    use super::*;

    pub fn create_freelance_profile(ctx: Context<CreateFreelanceProfile>,name: String,email: String) -> Result<()>{
        instructions::create_freelance_profile(ctx,name,email)?;
        Ok(())
    }

    pub fn update_freelance_profile(ctx: Context<UpdateFreelanceProfile>,name: String,email: String,bio:String,skills: Vec<Skill>,country: String,linkedin: String) -> Result<()> {
        instructions::update_freelance_profile(ctx,name,email,bio,skills,country,linkedin)?;
        Ok(())
    }

    pub fn delete_freelance_profile(ctx: Context<DeleteFreelanceProfile>) -> Result<()>{
        instructions::delete_freelance_profile(ctx)?;
        Ok(())
    }

    pub fn create_client_profile(ctx: Context<CreateClientProfile>,name: String,email: String) -> Result<()>{
        instructions::create_client_profile(ctx,name,email)?;
        Ok(())
    }

    pub fn update_client_profile(ctx: Context<UpdateClientProfile>,name: String,email: String,bio:String,country: String,linkedin: String) -> Result<()> {
        instructions::update_client_profile(ctx,name,email,bio,country,linkedin)?;
        Ok(())
    }

    pub fn delete_client_profile(ctx: Context<DeleteClientProfile>) -> Result<()>{
        instructions::delete_client_profile(ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
