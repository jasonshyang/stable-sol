use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_collateral_ratio: u64,
    pub bump: u8,
    pub bump_mint: u8,
    pub is_initialized: bool,
}
