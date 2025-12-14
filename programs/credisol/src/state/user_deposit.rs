use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserDeposit {
    pub user: Pubkey,
    pub bank: Pubkey,
    pub amount: u64,
}
