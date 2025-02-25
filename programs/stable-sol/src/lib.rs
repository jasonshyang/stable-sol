use anchor_lang::prelude::*;

pub mod error;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("R27zfMYiacB7PNCtkrZkRCk5bgWnniLg1AaJbxkdBFm");

#[program]
pub mod stable_sol {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
        min_collateral_ratio: u64,
    ) -> Result<()> {
        handle_initialize_config(
            ctx,
            liquidation_threshold,
            liquidation_bonus,
            min_collateral_ratio,
        )
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
        min_collateral_ratio: u64,
    ) -> Result<()> {
        handle_update_config(
            ctx,
            liquidation_threshold,
            liquidation_bonus,
            min_collateral_ratio,
        )
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        collateral_amount: u64,
        mint_amount: u64,
    ) -> Result<()> {
        msg!("Deposit collateral and mint tokens");
        handle_deposit_collateral_and_mint_tokens(ctx, collateral_amount, mint_amount)
    }

    pub fn withdraw_collateral_and_burn_tokens(
        ctx: Context<WithdrawCollateralAndBurnTokens>,
        collateral_amount: u64,
        burn_amount: u64,
    ) -> Result<()> {
        handle_withdraw_collateral_and_burn_tokens(ctx, collateral_amount, burn_amount)
    }

    pub fn liquidate(ctx: Context<Liquidate>, burn_amount: u64) -> Result<()> {
        handle_liquidate(ctx, burn_amount)
    }

    pub fn initialize_permissioned_account(
        ctx: Context<InitializePermissionedAccount>,
        permissioned_user: Pubkey,
        is_gatekeeper: bool,
        is_minter: bool,
        is_burner: bool,
        is_liquidator: bool,
    ) -> Result<()> {
        handle_initialize_permissioned_account(
            ctx,
            permissioned_user,
            is_gatekeeper,
            is_minter,
            is_burner,
            is_liquidator,
        )
    }
}
