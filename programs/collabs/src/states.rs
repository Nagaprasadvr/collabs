use anchor_lang::prelude::*;
use std::mem;

#[account]
pub struct XpPoolAccount {
    pub leader: Pubkey,
    pub xp: u64,
}

#[account]
pub struct ContributorAccount {
    pub contributor_pubkey: Pubkey,
    pub xp_pool_pubkey: Pubkey,
    pub xp: u64,
}

impl XpPoolAccount {
    pub const SPACE: usize = 8usize + mem::size_of::<XpPoolAccount>();
}

impl ContributorAccount {
    pub const SPACE: usize = 8usize + mem::size_of::<ContributorAccount>();
}
