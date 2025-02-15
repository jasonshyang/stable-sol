use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::state::Config;
use crate::utils::constants;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = constants::ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [constants::CONFIG_ACCOUNT_SEED],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [constants::MINT_ACCOUNT_SEED],
        bump,
        mint::decimals = constants::MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}


pub fn handle_initialize_config(
    ctx: Context<InitializeConfig>,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    min_collateral_ratio: u64,
) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold,
        liquidation_bonus,
        min_collateral_ratio,
        bump: ctx.bumps.config_account,
        bump_mint: ctx.bumps.mint_account,
    };

    Ok(())
}