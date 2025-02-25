use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct PermissionedAccount {
    pub authority: Pubkey,
    pub is_gatekeeper: bool,
    pub is_minter: bool,
    pub is_burner: bool,
    pub is_liquidator: bool,
    pub is_initialized: bool,
}