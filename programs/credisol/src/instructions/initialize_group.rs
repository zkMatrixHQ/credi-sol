use crate::{constants::ANCHOR_DISCRIMINATOR, state::Group};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeGroup<'info> {
    #[account(
	    init,
	    payer = admin,
	    space = ANCHOR_DISCRIMINATOR + Group::INIT_SPACE
    )]
    pub group: Account<'info, Group>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeGroup<'info> {
    pub fn initialize_group_ix(&mut self) -> Result<()> {
        self.group.set_inner(Group {
            admin: self.admin.key(),
        });
        Ok(())
    }
}
