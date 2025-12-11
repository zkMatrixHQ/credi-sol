#![allow(unexpected_cfgs,deprecated)]
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use state::*;
pub use instructions::*;

declare_id!("FJbMeAjXaPnZ8JV5PYKwiT7QFHMz5xyAvuthae6VJdq");

#[program]
pub mod credisol {
    use super::*;

    pub fn initialize_group(ctx: Context<InitializeGroup>) -> Result<()> {
        initialize_group::initialize_group_instruction(ctx)
    }

    pub fn create_bank(ctx: Context<CreateBank>) -> Result<()> {
        create_bank::create_bank_instruction(ctx)
    }

    pub fn supply_liquidity(ctx: Context<SupplyLiquidity>, amount: u64) -> Result<()> {
        supply_liquidity::supply_liquidity_instruction(ctx, amount)
    }
}
