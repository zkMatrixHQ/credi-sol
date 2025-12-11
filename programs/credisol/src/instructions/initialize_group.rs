use anchor_lang::prelude::*;
use crate::state::Group;

#[derive(Accounts)]
pub struct InitializeGroup<'info> {
    #[account(init, payer = admin, space = Group::INIT_SPACE)]
    pub group: Account<'info, Group>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,

}

pub fn initialize_group_instruction(ctx: Context<InitializeGroup>) -> Result<()> {
    let group = &mut ctx.accounts.group;
    group.admin = ctx.accounts.admin.key();
    Ok(())
}
