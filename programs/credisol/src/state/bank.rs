use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_deposits: u64,
}
