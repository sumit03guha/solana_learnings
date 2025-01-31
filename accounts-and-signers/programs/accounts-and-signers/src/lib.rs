use anchor_lang::prelude::*;

declare_id!("GgupEyyqv3EXoBjKKXFAPiTzjH3aHBsXcjhAyisUd2q1");

#[program]
pub mod accounts_and_signers {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn call_from_signer(ctx:Context<Initialize>) -> Result<()> {
        let signer = &ctx.accounts.signer;
        msg!("Namaste from {:?}", signer);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>
}
