use anchor_lang::prelude::*;

use crate::state::Config;
use crate::utils::constants;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::CONFIG_ACCOUNT_SEED],
        bump = config_account.bump,
        has_one = authority
    )]
    pub config_account: Account<'info, Config>,
}

pub fn handle_update_config(
    ctx: Context<UpdateConfig>,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    min_collateral_ratio: u64,
) -> Result<()> {
    ctx.accounts.config_account.liquidation_threshold = liquidation_threshold;
    ctx.accounts.config_account.liquidation_bonus = liquidation_bonus;
    ctx.accounts.config_account.min_collateral_ratio = min_collateral_ratio;

    Ok(())
}
