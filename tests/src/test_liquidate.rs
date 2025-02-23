use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, signer::Signer},
    Client, 
    Cluster
};

use crate::utils::{initialize_config, setup_test_context, update_config};

#[test]
fn test_liquidate() {
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

    let collateral_amount = 500_000_000;
    let burn_amount = 500_000_000;

    let withdraw_tx = program
        .request()
        .accounts(stable_sol::accounts::WithdrawCollateralAndBurnTokens{
            user: ctx.user.pubkey(),
            config_account: ctx.config_account,
            mint_account: ctx.mint_account,
            collateral_account: ctx.collateral_account,
            sol_account: ctx.sol_account,
            token_account: ctx.token_account,
            price_update: ctx.price_feed_account,
            token_program: ctx.token_program_id,
            system_program: ctx.system_program_id,
        })
        .args(stable_sol::instruction::WithdrawCollateralAndBurnTokens {
            collateral_amount,
            burn_amount,
        })
        .send()
        .expect("Withdraw failed");

    println!("Withdraw Transaction signature {}", withdraw_tx);
    
    let update_config_tx = update_config(
        &program,
        &ctx,
        50,
        10,
        200,
    ).expect("Failed to update config");

    println!("Update Config Transaction signature {}", update_config_tx);

    let liquidate_amount = 500_000_000;

    let liquidate_tx = program
        .request()
        .accounts(stable_sol::accounts::Liquidate {
            user: ctx.user.pubkey(),
            config_account: ctx.config_account,
            mint_account: ctx.mint_account,
            collateral_account: ctx.collateral_account,
            sol_account: ctx.sol_account,
            token_account: ctx.token_account,
            price_update: ctx.price_feed_account,
            token_program: ctx.token_program_id,
            system_program: ctx.system_program_id,
        })
        .args(stable_sol::instruction::Liquidate {
            burn_amount: liquidate_amount,
        })
        .send()
        .expect("Liquidate failed");

    println!("Liquidate Transaction signature {}", liquidate_tx);
}