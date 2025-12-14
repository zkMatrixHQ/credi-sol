#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("FJbMeAjXaPnZ8JV5PYKwiT7QFHMz5xyAvuthae6VJdq");

#[program]
pub mod credisol {
    use super::*;

    pub fn initialize_group(ctx: Context<InitializeGroup>) -> Result<()> {
        ctx.accounts.initialize_group_ix()
    }

    pub fn create_bank(ctx: Context<CreateBank>) -> Result<()> {
        ctx.accounts.create_bank_ix()
    }

    pub fn supply_liquidity(ctx: Context<SupplyLiquidity>, amount: u64) -> Result<()> {
        ctx.accounts.supply_liquidity_ix(amount)
    }
}
