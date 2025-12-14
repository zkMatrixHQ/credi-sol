use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::LendingError;
use crate::state::{Bank, UserDeposit};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SupplyLiquidity<'info> {
    #[account(mut)]
    pub bank: Account<'info, Bank>,

    #[account(
        mut,
        seeds = [b"supply", user.key().as_ref(), bank.key().as_ref()],
        bump
    )]
    pub user_deposit: Account<'info, UserDeposit>,

    #[account(
        mut,
        constraint = user_token_account.mint == bank.mint @ LendingError::Unauthorized,
        constraint = user_token_account.owner == user.key() @ LendingError::Unauthorized
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault.key() == bank.vault @ LendingError::Unauthorized,
        constraint = vault.mint == bank.mint @ LendingError::Unauthorized
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SupplyLiquidity<'info> {
    pub fn supply_liquidity_ix(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, LendingError::InvalidAmount);

        let transfer_accounts = Transfer {
            from: self.user_token_account.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };

        token::transfer(
            CpiContext::new(self.token_program.to_account_info(), transfer_accounts),
            amount,
        )?;

        let bank = &mut self.bank;
        bank.total_deposits = bank
            .total_deposits
            .checked_add(amount)
            .ok_or(LendingError::MathOverflow)?;

        // Check if user_deposit account needs to be initialized
        let user_deposit_info = self.user_deposit.to_account_info();
        let is_new_account = user_deposit_info.data_is_empty();

        if is_new_account {
            let user_key = self.user.key();
            let bank_key = self.bank.key();
            let (pda, bump) = Pubkey::find_program_address(
                &[b"supply", user_key.as_ref(), bank_key.as_ref()],
                user_deposit_info.owner,
            );
            require!(pda == *user_deposit_info.key, LendingError::Unauthorized);

            // Initialize new account
            let bump_seed = [bump];
            let seeds = &[b"supply", user_key.as_ref(), bank_key.as_ref(), &bump_seed];
            let signer_seeds = &[&seeds[..]];

            let space = ANCHOR_DISCRIMINATOR + UserDeposit::INIT_SPACE;
            let rent = Rent::get()?;
            let lamports_required = rent.minimum_balance(space);

            anchor_lang::solana_program::program::invoke_signed(
                &anchor_lang::solana_program::system_instruction::create_account(
                    self.user.key,
                    user_deposit_info.key,
                    lamports_required,
                    space as u64,
                    user_deposit_info.owner,
                ),
                &[
                    self.user.to_account_info(),
                    user_deposit_info.clone(),
                    self.system_program.to_account_info(),
                ],
                signer_seeds,
            )?;

            // Set the account discriminator
            {
                let mut account_data = user_deposit_info.try_borrow_mut_data()?;
                let discriminator = UserDeposit::DISCRIMINATOR;
                account_data[..8].copy_from_slice(&discriminator);
            }

            // Initialize the account data using set_inner
            self.user_deposit.set_inner(UserDeposit {
                user: self.user.key(),
                bank: self.bank.key(),
                amount: 0,
            });
        }

        let user_deposit = &mut self.user_deposit;

        // Validate that the user_deposit belongs to the correct user and bank
        require!(
            user_deposit.user == self.user.key(),
            LendingError::Unauthorized
        );
        require!(
            user_deposit.bank == self.bank.key(),
            LendingError::Unauthorized
        );

        // Add to existing amount
        user_deposit.amount = user_deposit
            .amount
            .checked_add(amount)
            .ok_or(LendingError::MathOverflow)?;

        Ok(())
    }
}
