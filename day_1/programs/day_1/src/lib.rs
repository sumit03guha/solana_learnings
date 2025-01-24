use anchor_lang::prelude::*;

declare_id!("Gnd5cFrESQMbTELdpXwUf2mGWFtyroUSpESYywREMx98");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello Solana!!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
