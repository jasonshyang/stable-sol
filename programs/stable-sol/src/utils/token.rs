use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::{burn, mint_to, Burn, MintTo};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use super::constants;

pub fn mint_tokens<'info>(
    amount: u64,
    bump: u8,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[constants::MINT_ACCOUNT_SEED, &[bump]]];

    let cpi_accounts = MintTo {
        mint: mint_account.to_account_info(),
        to: token_account.to_account_info(),
        authority: mint_account.to_account_info(),
    };

    let cpi_ctx =
        CpiContext::new_with_signer(token_program.to_account_info(), cpi_accounts, signer_seeds);

    mint_to(cpi_ctx, amount)
}

pub fn deposit_sol<'info>(
    amount: u64,
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(system_program.to_account_info(), cpi_accounts);

    transfer(cpi_ctx, amount)
}

pub fn burn_tokens<'info>(
    amount: u64,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    authority: &Signer<'info>,
) -> Result<()> {
    let cpi_accounts = Burn {
        mint: mint_account.to_account_info(),
        from: token_account.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

    burn(cpi_ctx, amount)
}

pub fn withdraw_sol<'info>(
    amount: u64,
    bump: u8,
    user_key: &Pubkey,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[constants::SOL_ACCOUNT_SEED, user_key.as_ref(), &[bump]]];

    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.clone(),
    };

    let cpi_ctx =
        CpiContext::new_with_signer(system_program.to_account_info(), cpi_accounts, signer_seeds);

    transfer(cpi_ctx, amount)
}
