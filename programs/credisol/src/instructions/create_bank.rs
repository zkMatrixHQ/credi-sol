use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};
use crate::error::LendingError;
use crate::state::{Bank, Group};

#[derive(Accounts)]
pub struct CreateBank<'info> {
    #[account(
        init,
        payer = admin,
        space = Bank::INIT_SPACE,
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
        token::authority = bank
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_bank_instruction(ctx: Context<CreateBank>) -> Result<()> {
    require!(
        ctx.accounts.group.admin == ctx.accounts.admin.key(),
        LendingError::Unauthorized
    );

    let bank = &mut ctx.accounts.bank;
    bank.mint = ctx.accounts.mint.key();
    bank.vault = ctx.accounts.vault.key();
    bank.total_deposits = 0;

    Ok(())
}
