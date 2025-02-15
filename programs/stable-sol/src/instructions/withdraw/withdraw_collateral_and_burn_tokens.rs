use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::state::{Collateral, Config};
use crate::utils::{burn_tokens, check_collateral_ratio, constants, withdraw_sol};

#[derive(Accounts)]
pub struct WithdrawCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [constants::CONFIG_ACCOUNT_SEED],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [constants::COLLATERAL_ACCOUNT_SEED, user.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn handle_withdraw_collateral_and_burn_tokens(
    ctx: Context<WithdrawCollateralAndBurnTokens>,
    collateral_amount: u64,
    burn_amount: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;

    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - collateral_amount;
    collateral_account.minted_amount -= burn_amount;

    check_collateral_ratio(
        collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    burn_tokens(
        burn_amount,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        &ctx.accounts.user,
    )?;

    withdraw_sol(
        collateral_amount,
        collateral_account.bump,
        &ctx.accounts.user.key(),
        &ctx.accounts.sol_account,
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.system_program,
    )?;

    Ok(())
}
