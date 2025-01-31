use anchor_lang::prelude::*;

declare_id!("JAZBT7mKJMmwse95uJ7ZCz96i3h72mWZV8P5rKGdPDkJ");

#[program]
pub mod accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
