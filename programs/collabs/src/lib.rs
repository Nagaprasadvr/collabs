use anchor_lang::prelude::*;
mod errors;
use anchor_spl::{token::transfer, token::Transfer};
mod ix;
use ix::{CreateContributorAccount, CreateGitRepoXpPoolAccountWithStake, TransferXPToContributor};
use ix::{
    __client_accounts_create_contributor_account,
    __client_accounts_create_git_repo_xp_pool_account_with_stake,
    __client_accounts_transfer_xp_to_contributor,
};
mod states;
use errors::CollabsError;
const bonk_decimals: u64 = 1_00_000;
declare_id!("4HYr7M3ytiSoqr3Zh3iK1VcNNm7ZgrNikwmWYJdGMvw4");

#[program]
pub mod collabs {
    use super::*;

    pub fn create_git_repo_xp_pool_with_stake(
        ctx: Context<CreateGitRepoXpPoolAccountWithStake>,
        git_repo_url: String,
        stake_amount: u64,
    ) -> Result<()> {
        let git_repo_xp_pool = &mut ctx.accounts.git_repo_xp_pool_account;
        git_repo_xp_pool.leader = ctx.accounts.leader.key();
        git_repo_xp_pool.git_repo_url = git_repo_url;

        git_repo_xp_pool.total_bonk_in_stake = 0;
        git_repo_xp_pool.xp = 0;
        let cpi_accounts = Transfer {
            from: ctx.accounts.leader_token_acc.to_account_info(),
            to: ctx.accounts.bonk_escrow_token_acc.to_account_info(),
            authority: ctx.accounts.leader.to_account_info(),
        };
        let cpi_context =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        let result = transfer(cpi_context, stake_amount * bonk_decimals);
        match result {
            Ok(()) => {
                git_repo_xp_pool.total_bonk_in_stake = stake_amount;
                git_repo_xp_pool.xp = 100;
                msg!("Staking success!");
            }
            Err(_) => msg!("Err in transfer"),
        }
        Ok(())
    }

    pub fn create_contributor(
        ctx: Context<CreateContributorAccount>,
        contributor_git_name: String,
    ) -> Result<()> {
        let contributor_account = &mut ctx.accounts.contributor_account;
        contributor_account.contributor_pubkey = ctx.accounts.contributor.key();
        contributor_account.git_repo_xp_pool_pubkey = ctx.accounts.git_repo_xp_pool_account.key();
        contributor_account.contributor_git_name = contributor_git_name;
        contributor_account.xp = 0u64;
        Ok(())
    }

    pub fn transfer_xp_to_contributor(
        ctx: Context<TransferXPToContributor>,
        xp_to_transfer: u64,
    ) -> Result<()> {
        let TransferXPToContributor {
            contributor_account,
            git_repo_xp_pool_account,
            ..
        } = ctx.accounts;
        require!(
            git_repo_xp_pool_account.xp >= xp_to_transfer,
            CollabsError::NotEnoughXpsToTransfer
        );
        git_repo_xp_pool_account.xp -= xp_to_transfer;
        contributor_account.xp += xp_to_transfer;
        Ok(())
    }
}
