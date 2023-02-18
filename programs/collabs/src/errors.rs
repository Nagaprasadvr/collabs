use anchor_lang::prelude::*;

#[error_code]
pub enum CollabsError {
    #[msg("Not Enough Xps to tansfer!")]
    NotEnoughXpsToTransfer,
    #[msg("Token mint Mismatch!")]
    TokenMintMismatch,
    #[msg("Xp not Zero cannot Unstake")]
    CannotUnstake,
}
