#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("HQY5kLNtUJkEiArKxDyrkCKHBtK8pDFGUBifrGFjtLDt");

mod error_codes;
mod instructions;

use instructions::*;

#[program]
pub mod vijay {
    use super::*;

    pub fn initialize_state(
        ctx: Context<StateInfo>,
    ) -> Result<()> {
        instructions::initialize_state(ctx)
    }

    pub fn withdraw_balance(
        ctx: Context<Withdraw>,
    ) -> Result<()> {
        instructions::withdraw_balance(ctx)
    }

    pub fn initialize_client(
        ctx: Context<ClientInfo>,
        name: String,
        domain: String,
        required_skills: String,
        contact: String,
    ) -> Result<()> {
        instructions::initialize_client(ctx, name, domain, required_skills, contact)
    }

    pub fn initialize_freelancer(
        ctx: Context<FreelancerInfo>,
        name: String,
        domain: String,
        skills: String,
        contact: String,
    ) -> Result<()> {
        instructions::initialize_freelancer(ctx, name, domain, skills, contact)
    }

    pub fn initialize_project(
        ctx: Context<ProjectInfo>,
        name: String,
        description: String,
        url: String,
        budget: u64,
    ) -> Result<()> {
        instructions::initialize_project(ctx, name, description, url, budget)
    }

    pub fn project_escrow_setup(
        ctx: Context<ProjectSetupInfo>,
        project_id: u64,
        freelancer_key: Pubkey,
        amount: u64,
        total_tasks: u64,
    ) -> Result<()> {
        instructions::project_escrow_setup(ctx, project_id, freelancer_key, amount, total_tasks)
    }

    pub fn request_task_review(
        ctx: Context<TaskReviewInfo>,
        project_name: String,
        freelancer_project_id: u64,
        url: String,
    ) -> Result<()> {
        instructions::request_task_review(ctx, project_name, freelancer_project_id, url)
    }

    pub fn review_task_process(
        ctx: Context<TaskReviewProcess>,
        project_id: u64,
        approve: bool
    ) -> Result<()> {
        instructions::review_task_process(ctx, project_id, approve)
    }

    pub fn withdraw_project(
        ctx: Context<WithdrawInfo>,
        project_id: u64
    ) -> Result<()> {
        instructions::withdraw_project(ctx, project_id)
    }

    pub fn transfer_project(
        ctx: Context<TransferInfo>,
        project_id: u64,
        freelancer: Pubkey,
    ) -> Result<()> {
        instructions::transfer_project(ctx, project_id, freelancer)
    }
}
