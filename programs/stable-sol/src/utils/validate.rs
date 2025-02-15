use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::error::ErrorCode;
use crate::{
    state::{Collateral, Config},
    utils::calc_collateral_ratio,
};

pub fn validate_collateral_above_threshold<'info>(
    collateral_account: &Account<'info, Collateral>,
    config: &Account<'info, Config>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> Result<()> {
    let collateral_ratio = calc_collateral_ratio(collateral_account, config, price_update)?;

    require!(
        collateral_ratio >= config.min_collateral_ratio,
        ErrorCode::CollateralRatioTooLow
    );

    Ok(())
}

pub fn validate_collateral_below_threshold<'info>(
    collateral_account: &Account<'info, Collateral>,
    config: &Account<'info, Config>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> Result<()> {
    let collateral_ratio = calc_collateral_ratio(collateral_account, config, price_update)?;

    require!(
        collateral_ratio < config.min_collateral_ratio,
        ErrorCode::CollateralRatioTooHigh
    );

    Ok(())
}
