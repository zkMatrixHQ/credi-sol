use anchor_lang::prelude::*;

#[error_code]
pub enum LendingError {
    #[msg("Custom error message")]
    Unauthorized,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Math overflow")]
    MathOverflow,
}
