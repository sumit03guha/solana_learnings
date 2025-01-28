use anchor_lang::prelude::*;

declare_id!("4yEtV8g1oNJEEY3Vw2qbzQ1UpAP4AqCpTBsWRahFmQUx");

#[program]
pub mod tryrust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
