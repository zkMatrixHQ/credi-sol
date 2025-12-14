use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Group {
    pub admin: Pubkey,
}