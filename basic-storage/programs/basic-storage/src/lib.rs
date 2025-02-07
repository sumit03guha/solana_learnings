use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fo9joCKSfsYSweJYs3TnigXtMFTkNwUDbMkacNTa2rwh");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init,
              payer = signer,
              space = size_of::<Storage>() + 8,
              seeds = [],
              bump)]
    pub storage: Account<'info, Storage>,

    pub system_program: Program<'info, System>
}

#[account]
pub struct Storage {
    x: u64
}
