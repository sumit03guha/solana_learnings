use anchor_lang::prelude::*;

declare_id!("GgupEyyqv3EXoBjKKXFAPiTzjH3aHBsXcjhAyisUd2q1");

#[program]
pub mod accounts_and_signers {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
