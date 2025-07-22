// instructions/mod.rs
pub mod create_client;
pub mod create_freelancer;
pub mod update_client;
pub mod update_freelancer;
pub mod delete_client;
pub mod delete_freelancer;
pub mod create_project;
pub mod fund_project;
pub mod approve_project;
pub mod complete_project;
pub mod submit_proposal;
pub mod respond_to_proposal;
pub mod submit_work;
pub mod approve_work_and_pay;

// 1.  re-export every handler
pub use create_client::*;
pub use create_freelancer::*;
pub use update_client::*;
pub use update_freelancer::*;
pub use delete_client::*;
pub use delete_freelancer::*;
pub use create_project::*;
pub use fund_project::*;
pub use approve_project::*;
pub use complete_project::*;
pub use submit_proposal::*;
pub use respond_to_proposal::*;
pub use submit_work::*;
pub use approve_work_and_pay::*;

// 2.  ALSO re-export every Accounts struct
pub use create_client::CreateClientProfile;
pub use create_freelancer::CreateFreelancerProfile;
pub use update_client::UpdateClientProfile;
pub use update_freelancer::UpdateFreelancerProfile;
pub use delete_client::DeleteClientProfile;
pub use delete_freelancer::DeleteFreelancerProfile;
pub use create_project::CreateProject;
pub use fund_project::FundProject;
pub use approve_project::ApproveProject;
pub use complete_project::CompleteProject;
pub use submit_proposal::SubmitProposal;
pub use respond_to_proposal::RespondToProposal;
pub use submit_work::SubmitWork;
pub use approve_work_and_pay::ApproveWorkAndPay;