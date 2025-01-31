use anchor_lang::prelude::*;

declare_id!("9dw8AoNJsdkDvFNP7LCsRW37jpUQLNjUVYs9EozRtRkz");

#[program]
pub mod events {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
