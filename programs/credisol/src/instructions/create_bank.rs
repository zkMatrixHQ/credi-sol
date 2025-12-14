use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::LendingError;
use crate::state::{Bank, Group};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateBank<'info> {
    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + Bank::INIT_SPACE,
        seeds = [b"bank", mint.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(mut)]
    pub group: Account<'info, Group>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        token::mint = mint,
        token::authority = bank,
        constraint = vault.mint == mint.key() @ LendingError::Unauthorized
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateBank<'info> {
    pub fn create_bank_ix(&mut self) -> Result<()> {
        require!(
            self.group.admin == self.admin.key(),
            LendingError::Unauthorized
        );

        self.bank.set_inner(Bank {
            mint: self.mint.key(),
            vault: self.vault.key(),
            total_deposits: 0,
        });
        Ok(())
    }
}
