use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signer::Signer},
    Client, 
    Cluster
};

use crate::utils::{initialize_config, setup_test_context};

#[test]
fn test_deposit_and_mint() {
    let ctx = setup_test_context(50, 10, 100);
    
    let client = Client::new_with_options(Cluster::Localnet, &ctx.user, CommitmentConfig::confirmed());
    let program = client.program(ctx.program_id).unwrap();

    let init_tx = initialize_config(&program, &ctx)
        .expect("Failed to initialize config");

    println!("Initialize Config Transaction signature {}", init_tx);

    let collateral_amount = 1_500_000_000;
    let mint_amount = 1_000_000_000;

    let deposit_tx = program
        .request()
        .accounts(stable_sol::accounts::DepositCollateralAndMintTokens{
            user: ctx.user.pubkey(),
            config_account: ctx.config_account,
            mint_account: ctx.mint_account,
            collateral_account: ctx.collateral_account,
            sol_account: ctx.sol_account,
            token_account: ctx.token_account,
            price_update: ctx.price_feed_account,
            token_program: ctx.token_program_id,
            associated_token_program: ctx.associated_token_program_id,
            system_program: ctx.system_program_id,
        })
        .args(stable_sol::instruction::DepositCollateralAndMintTokens {
            collateral_amount,
            mint_amount,
        })
        .send()
        .expect("Deposit failed");

    println!("Deposit Transaction signature {}", deposit_tx);
}