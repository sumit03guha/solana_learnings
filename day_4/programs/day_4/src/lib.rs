use anchor_lang::prelude::*;

declare_id!("EabHrNd6S9eftdasCnAiz4zKosGWZcS98upxr5o5xqCf");

#[program]
pub mod day_4 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
