use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::state::{Collateral, Config};
use crate::utils::{check_collateral_ratio, constants, deposit_sol, mint_tokens};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
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
        init_if_needed,
        payer = user,
        space = constants::ANCHOR_DISCRIMINATOR + Collateral::INIT_SPACE,
        seeds = [constants::COLLATERAL_ACCOUNT_SEED, user.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [constants::SOL_ACCOUNT_SEED, user.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_account,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralAndMintTokens>,
    collateral_amount: u64,
    mint_amount: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;

    if !collateral_account.is_initialized {
        collateral_account.owner = ctx.accounts.user.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.is_initialized = true;
    }

    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() + collateral_amount;
    collateral_account.minted_amount += mint_amount;

    check_collateral_ratio(
        collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    deposit_sol(
        collateral_amount,
        &ctx.accounts.user,
        &ctx.accounts.sol_account,
        &ctx.accounts.system_program,
    )?;

    mint_tokens(
        mint_amount,
        ctx.accounts.config_account.bump_mint,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}
