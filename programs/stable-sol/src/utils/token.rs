use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::token_interface::{Mint, TokenAccount,Token2022};


use super::constants;

pub fn mint_token<'info>(
    amount: u64,
    bump: u8,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        constants::MINT_ACCOUNT_SEED,
        &[bump],
    ]];

    let cpi_accounts = MintTo {
        mint: mint_account.to_account_info(),
        to: token_account.to_account_info(),
        authority: token_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

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
