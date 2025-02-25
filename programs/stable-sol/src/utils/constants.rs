use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const CONFIG_ACCOUNT_SEED: &[u8] = b"config";
pub const MINT_ACCOUNT_SEED: &[u8] = b"mint";
pub const COLLATERAL_ACCOUNT_SEED: &[u8] = b"collateral";
pub const SOL_ACCOUNT_SEED: &[u8] = b"sol";
pub const PERMISSIONED_ACCOUNT_SEED: &[u8] = b"permissioned";

#[constant]
pub const MINT_DECIMALS: u8 = 6;
pub const PYTH_SOL_USD_FEED_ID: &str =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
pub const PYTH_MAX_DELAY: u64 = 60;
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10;
