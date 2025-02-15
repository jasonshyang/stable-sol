use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::error::ErrorCode;
use crate::state::{Collateral, Config};
use crate::utils::constants;

pub fn calc_collateral_ratio<'info>(
    collateral_account: &Account<'info, Collateral>,
    config: &Account<'info, Config>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> Result<u64> {
    let collateral_usd_value = get_usd_value(&collateral_account.lamport_balance, price_update)?;

    let collateral_usd_value_after_liquidation_threshold =
        collateral_usd_value * config.liquidation_threshold / 100;

    if collateral_account.minted_amount == 0 {
        msg!("Max collateral ratio");
        return Ok(u64::MAX);
    }

    let collateral_ratio =
        collateral_usd_value_after_liquidation_threshold / collateral_account.minted_amount;

    Ok(collateral_ratio)
}

pub fn calc_liquidation_amount_from_burn_amount<'info>(
    burn_amount: &u64,
    config: &Account<'info, Config>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> Result<u64> {
    let lamport_value = get_lamport_value(burn_amount, price_update)?;
    let liquidation_bonus = lamport_value * config.liquidation_bonus / 100;

    Ok(lamport_value + liquidation_bonus)
}

pub fn get_usd_value(lamport_amount: &u64, price_update: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(constants::PYTH_SOL_USD_FEED_ID)?;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        constants::PYTH_MAX_DELAY,
        &feed_id,
    )?;

    require!(price.price > 0, ErrorCode::InvalidPrice);

    let price_in_usd = price.price as u128 * constants::PRICE_FEED_DECIMAL_ADJUSTMENT;
    let usd_amount = (*lamport_amount as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);

    Ok(usd_amount as u64)
}

pub fn get_lamport_value(usd_amount: &u64, price_update: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(constants::PYTH_SOL_USD_FEED_ID)?;
    let price = price_update.get_price_no_older_than(
        &Clock::get()?,
        constants::PYTH_MAX_DELAY,
        &feed_id,
    )?;

    require!(price.price > 0, ErrorCode::InvalidPrice);

    let price_in_usd = price.price as u128 * constants::PRICE_FEED_DECIMAL_ADJUSTMENT;
    let lamport_amount = (*usd_amount as u128 * LAMPORTS_PER_SOL as u128) / price_in_usd;

    Ok(lamport_amount as u64)
}
