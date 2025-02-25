use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signer::Signer},
    Client, 
    Cluster
};
use stable_sol::utils::constants;
use crate::utils::{initialize_config, setup_test_context};

#[test]
fn test_initialize_permissioned_account() {
    let ctx = setup_test_context(50, 10, 100);
    
    let client = Client::new_with_options(Cluster::Localnet, &ctx.user, CommitmentConfig::confirmed());
    let program = client.program(ctx.program_id).unwrap();

    let init_tx = initialize_config(&program, &ctx)
        .expect("Failed to initialize config");

    println!("Initialize Config Transaction signature {}", init_tx);

    let permissioned_user_pubkey = Pubkey::new_unique();

    let permissioned_seed = &[
        constants::PERMISSIONED_ACCOUNT_SEED,
        permissioned_user_pubkey.as_ref(),
    ];

    let permissioned_account = Pubkey::find_program_address(
        permissioned_seed,
        &ctx.program_id,
    ).0;

    let init_permissioned_tx = program
        .request()
        .accounts(stable_sol::accounts::InitializePermissionedAccount{
            authority: ctx.user.pubkey(),
            permissioned_account: permissioned_account,
            config_account: ctx.config_account,
            system_program: ctx.system_program_id,
        })
        .args(stable_sol::instruction::InitializePermissionedAccount {
            permissioned_user: permissioned_user_pubkey,
            is_gatekeeper: true,
            is_minter: true,
            is_burner: true,
            is_liquidator: true,
        })
        .send()
        .expect("Initialize Permissioned Account failed");

    println!("Initialize Permissioned Account Transaction signature {}", init_permissioned_tx);
}