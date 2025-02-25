use anchor_lang::prelude::*;

use crate::state::{Config, PermissionedAccount};
use crate::utils::constants;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(permissioned_user: Pubkey)]
pub struct InitializePermissionedAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = constants::ANCHOR_DISCRIMINATOR + PermissionedAccount::INIT_SPACE,
        seeds = [constants::PERMISSIONED_ACCOUNT_SEED, permissioned_user.as_ref()],
        bump,
    )]
    pub permissioned_account: Account<'info, PermissionedAccount>,

    #[account(
        seeds = [constants::CONFIG_ACCOUNT_SEED],
        bump = config_account.bump,
        has_one = authority
    )]
    pub config_account: Box<Account<'info, Config>>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_permissioned_account(
    ctx: Context<InitializePermissionedAccount>,
    permissioned_user: Pubkey,
    is_gatekeeper: bool,
    is_minter: bool,
    is_burner: bool,
    is_liquidator: bool,
) -> Result<()> {
    require!(!ctx.accounts.permissioned_account.is_initialized, ErrorCode::AccountAlreadyInitialized);

    *ctx.accounts.permissioned_account = PermissionedAccount {
        authority: permissioned_user,
        is_gatekeeper,
        is_minter,
        is_burner,
        is_liquidator,
        is_initialized: true,
    };

    Ok(())
}