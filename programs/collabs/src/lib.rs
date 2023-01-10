use anchor_lang::prelude::*;
mod errors;
mod ix;
use ix::{CreateContributorAccount, CreateXpPoolAccount, TransferXPToContributor};
use ix::{
    __client_accounts_create_contributor_account, __client_accounts_create_xp_pool_account,
    __client_accounts_transfer_xp_to_contributor,
};
mod states;
use errors::CollabsError;

declare_id!("4HYr7M3ytiSoqr3Zh3iK1VcNNm7ZgrNikwmWYJdGMvw4");

#[program]
pub mod collabs {
    use super::*;

    pub fn create_xp_pool(ctx: Context<CreateXpPoolAccount>) -> Result<()> {
        let xp_pool = &mut ctx.accounts.xp_pool_account;
        xp_pool.xp = 100u64;
        xp_pool.leader = ctx.accounts.leader.key();
        Ok(())
    }

    pub fn create_contributor(ctx: Context<CreateContributorAccount>) -> Result<()> {
        let contributor_account = &mut ctx.accounts.contributor_account;
        contributor_account.contributor_pubkey = ctx.accounts.contributor.key();
        contributor_account.xp_pool_pubkey = ctx.accounts.xp_pool_account.key();
        contributor_account.xp = 0u64;
        Ok(())
    }

    pub fn transfer_xp_to_contributor(
        ctx: Context<TransferXPToContributor>,
        xp_to_transfer: u64,
    ) -> Result<()> {
        let TransferXPToContributor {
            contributor_account,
            xp_pool_account,
            ..
        } = ctx.accounts;
        require!(
            xp_pool_account.xp >= xp_to_transfer,
            CollabsError::NotEnoughXpsToTransfer
        );
        xp_pool_account.xp -= xp_to_transfer;
        contributor_account.xp += xp_to_transfer;
        Ok(())
    }
}
