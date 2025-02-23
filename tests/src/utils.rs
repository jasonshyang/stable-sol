use std::str::FromStr;
use anchor_client::{
    anchor_lang::system_program, solana_sdk::{
        pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signature}, signer::Signer
    }, ClientError, Program
};
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use pyth_solana_receiver_sdk::price_update::get_feed_id_from_hex;

use stable_sol::utils::constants;

const PROGRAM_ID: &str = "R27zfMYiacB7PNCtkrZkRCk5bgWnniLg1AaJbxkdBFm";
const PYTH_PUSH_ORACLE_PROGRAM_ID: &str = "pythWSnswVUd12oZpeFP8e9CVaEqJg25g1Vtc2biRsT";
const PYTH_SOL_USD_PRICE_FEED: &str = "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
const PYTH_SHARD_ID: u16 = 0;

pub struct TestContext {
    pub user: Keypair,
    pub mint_account: Pubkey,
    pub collateral_account: Pubkey,
    pub sol_account: Pubkey,
    pub token_account: Pubkey,
    pub config_account: Pubkey,
    pub price_feed_account: Pubkey,
    pub program_id: Pubkey,
    pub token_program_id: Pubkey,
    pub associated_token_program_id: Pubkey,
    pub system_program_id: Pubkey,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_collateral_ratio: u64,
}

pub fn setup_test_context(
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    min_collateral_ratio: u64,
) -> TestContext {
    // Initialize program ids
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let token_program_id = anchor_spl::token_2022::ID;
    let system_program_id = system_program::ID;
    let associated_token_program_id = anchor_spl::associated_token::ID;

    // Initialize Pyth price feed
    let default_push_oracle_program_id = Pubkey::from_str(PYTH_PUSH_ORACLE_PROGRAM_ID).unwrap();
    let sol_usd_price_feed_id = get_feed_id_from_hex(PYTH_SOL_USD_PRICE_FEED).unwrap();
    let price_feed_account = Pubkey::find_program_address(
        &[&PYTH_SHARD_ID.to_le_bytes(), sol_usd_price_feed_id.as_ref()],
        &default_push_oracle_program_id,
    ).0;

    // Initialize user account
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let user = read_keypair_file(&anchor_wallet).unwrap();
    let user_pubkey = user.pubkey();

    // Initialize context accounts
    let config_account_seed = &[constants::CONFIG_ACCOUNT_SEED];
    let mint_account_seed = &[constants::MINT_ACCOUNT_SEED];
    let collateral_account_seed = &[constants::COLLATERAL_ACCOUNT_SEED, user_pubkey.as_ref()];
    let sol_account_seed = &[constants::SOL_ACCOUNT_SEED, user_pubkey.as_ref()];

    let config_account = Pubkey::find_program_address(config_account_seed, &program_id).0;
    let mint_account = Pubkey::find_program_address(mint_account_seed, &program_id).0;
    let collateral_account = Pubkey::find_program_address(collateral_account_seed, &program_id).0;
    let sol_account = Pubkey::find_program_address(sol_account_seed, &program_id).0;
    let token_account = get_associated_token_address_with_program_id(
        &user.pubkey(),
        &mint_account,
        &token_program_id,
    );

    TestContext {
        user,
        mint_account,
        collateral_account,
        sol_account,
        token_account,
        config_account,
        price_feed_account,
        program_id,
        token_program_id,
        associated_token_program_id,
        system_program_id,
        liquidation_threshold,
        liquidation_bonus,
        min_collateral_ratio,
    }
}

pub fn initialize_config(
    program: &Program<&Keypair>,
    ctx: &TestContext,
) -> Result<Signature, ClientError> {
    program
        .request()
        .accounts(stable_sol::accounts::InitializeConfig {
            authority: ctx.user.pubkey(),
            config_account: ctx.config_account,
            mint_account: ctx.mint_account,
            system_program: ctx.system_program_id,
            token_program: ctx.token_program_id,
        })
        .args(stable_sol::instruction::InitializeConfig {
            liquidation_threshold: ctx.liquidation_threshold,
            liquidation_bonus: ctx.liquidation_bonus,
            min_collateral_ratio: ctx.min_collateral_ratio,
        })
        .send()
}

pub fn update_config(
    program: &Program<&Keypair>,
    ctx: &TestContext,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    min_collateral_ratio: u64,
) -> Result<Signature, ClientError> {
    program
        .request()
        .accounts(stable_sol::accounts::UpdateConfig {
            config_account: ctx.config_account,
        })
        .args(stable_sol::instruction::UpdateConfig {
            liquidation_threshold,
            liquidation_bonus,
            min_collateral_ratio,
        })
        .send()
}