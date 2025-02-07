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
pub struct Initialize<> {}



