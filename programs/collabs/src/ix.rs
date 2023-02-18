use crate::instruction;

use super::errors::*;
use super::states::{ContributorAccount, GitRepoXpPoolAccount};
use anchor_lang::prelude::*;
use anchor_spl::{token::Mint, token::Token, token::TokenAccount};

#[derive(Accounts)]
pub struct CreateGitRepoXpPoolAccountWithStake<'info> {
    #[account(init,payer=leader,seeds=["git_repo_xp_pool".as_bytes(),leader.key().as_ref()],bump,space=GitRepoXpPoolAccount::SPACE)]
    pub git_repo_xp_pool_account: Account<'info, GitRepoXpPoolAccount>,
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(mut,constraint = leader_token_acc.mint == bonk_mint.key() @CollabsError::TokenMintMismatch)]
    pub leader_token_acc: Account<'info, TokenAccount>,
    pub bonk_mint: Account<'info, Mint>,
    #[account(init,payer=leader,seeds = ["total_bonk_stake".as_bytes(),leader.key().as_ref()],bump,token::mint = bonk_mint,token::authority = bonk_escrow_token_acc)]
    pub bonk_escrow_token_acc: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
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
    pub git_repo_xp_pool_account: Account<'info, GitRepoXpPoolAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferXPToContributor<'info> {
    #[account(mut)]
    pub contributor_account: Account<'info, ContributorAccount>,
    #[account(mut,has_one = leader)]
    pub git_repo_xp_pool_account: Account<'info, GitRepoXpPoolAccount>,
    pub leader: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct CloseGitRepoXpPoolAccountWithUnStake<'info> {
    #[account(mut,close=leader)]
    pub git_repo_xp_pool_account: Account<'info, GitRepoXpPoolAccount>,
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(mut,close = leader,constraint = leader_token_acc.mint == bonk_mint.key() @CollabsError::TokenMintMismatch)]
    pub leader_token_acc: Account<'info, TokenAccount>,
    pub bonk_mint: Account<'info, Mint>,
    #[account(mut,seeds = ["total_bonk_stake".as_bytes(),leader.key().as_ref()],bump,token::mint = bonk_mint,token::authority = bonk_escrow_token_acc)]
    pub bonk_escrow_token_acc: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
