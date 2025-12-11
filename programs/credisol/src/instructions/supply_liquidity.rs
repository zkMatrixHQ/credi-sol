use anchor_lang::prelude::*;
use crate::error::LendingError;
use crate::state::{Bank, UserDeposit};
use anchor_spl::token::{self, TokenAccount, Token, Transfer};

#[derive(Accounts)]
pub struct SupplyLiquidity<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        payer = user,
        space = UserDeposit::INIT_SPACE,
        seeds = [b"supply", user.key().as_ref(), bank.key().as_ref()],
        bump
    )]
    pub user_deposit: Account<'info, UserDeposit>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn supply_liquidity_instruction(ctx: Context<SupplyLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, LendingError::InvalidAmount);

    let transfer_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts),
        amount,
    )?;

    let bank = &mut ctx.accounts.bank;
    bank.total_deposits = bank.total_deposits.checked_add(amount)
        .ok_or(LendingError::MathOverflow)?;

    let user_deposit = &mut ctx.accounts.user_deposit;
    user_deposit.user = ctx.accounts.user.key();
    user_deposit.bank = ctx.accounts.bank.key();
    user_deposit.amount = user_deposit.amount.checked_add(amount)
        .ok_or(LendingError::MathOverflow)?;

    Ok(())
}
