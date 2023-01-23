use anchor_lang::prelude::*;
use std::mem;

#[account]
pub struct GitRepoXpPoolAccount {
    pub leader: Pubkey,
    pub xp: u64,
    pub git_repo_url: String,
    pub total_bonk_in_stake: u64,
}

#[account]
pub struct ContributorAccount {
    pub contributor_pubkey: Pubkey,
    pub contributor_git_name: String,
    pub git_repo_xp_pool_pubkey: Pubkey,
    pub xp: u64,
}

impl GitRepoXpPoolAccount {
    pub const SPACE: usize = 8usize + mem::size_of::<GitRepoXpPoolAccount>();
}

impl ContributorAccount {
    pub const SPACE: usize = 8usize + mem::size_of::<ContributorAccount>();
}
