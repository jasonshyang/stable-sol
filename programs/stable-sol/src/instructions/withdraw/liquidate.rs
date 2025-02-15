use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    state::{Collateral, Config},
    utils::{
        burn_tokens, calc_liquidation_amount_from_burn_amount, constants,
        validate_collateral_below_threshold, withdraw_sol,
    },
};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [constants::CONFIG_ACCOUNT_SEED],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn handle_liquidate(ctx: Context<Liquidate>, burn_amount: u64) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;

    validate_collateral_below_threshold(
        collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    let liquidation_amount = calc_liquidation_amount_from_burn_amount(
        &burn_amount,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - liquidation_amount; //TODO: add checked_sub
    collateral_account.minted_amount -= burn_amount; //TODO: add checked_sub

    burn_tokens(
        burn_amount,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        &ctx.accounts.user,
    )?;

    withdraw_sol(
        liquidation_amount,
        collateral_account.bump_sol_account,
        &ctx.accounts.user.key(),
        &ctx.accounts.sol_account,
        &ctx.accounts.user.to_account_info(),
        &ctx.accounts.system_program,
    )?;

    Ok(())
}
