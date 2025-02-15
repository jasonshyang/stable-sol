use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("Collateral Ratio Below Minimum")]
    CollateralRatioTooLow
}