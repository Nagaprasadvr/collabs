use super::states::{ContributorAccount, XpPoolAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateXpPoolAccount<'info> {
    #[account(init,payer=leader,seeds=["xp_pool".as_bytes(),leader.key().as_ref()],bump,space=XpPoolAccount::SPACE)]
    pub xp_pool_account: Account<'info, XpPoolAccount>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateContributorAccount<'info> {
    #[account(init,payer=leader,seeds=["contributor".as_bytes(),contributor.key().as_ref()],bump,space=ContributorAccount::SPACE)]
    pub contributor_account: Account<'info, ContributorAccount>,
    #[account(mut)]
    pub leader: Signer<'info>,
    /// CHECK:contributor
    pub contributor: AccountInfo<'info>,
    pub xp_pool_account: Account<'info, XpPoolAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferXPToContributor<'info> {
    #[account(mut)]
    pub contributor_account: Account<'info, ContributorAccount>,
    #[account(mut,has_one = leader)]
    pub xp_pool_account: Account<'info, XpPoolAccount>,
    pub leader: Signer<'info>,
}
