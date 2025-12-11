use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Group {
    pub admin: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_deposits: u64,
}

#[account]
#[derive(InitSpace)]
pub struct UserDeposit {
    pub user: Pubkey,
    pub bank: Pubkey,
    pub amount: u64,
}
