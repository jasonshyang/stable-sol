use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Account Already Initialized")]
    AccountAlreadyInitialized,
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("Collateral Ratio Below Minimum")]
    CollateralRatioTooLow,
    #[msg("Collateral Ratio Above Minimum")]
    CollateralRatioTooHigh,
}
