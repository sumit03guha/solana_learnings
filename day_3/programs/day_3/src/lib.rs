use anchor_lang::prelude::*;

declare_id!("8dpGXb1gmJc7DNHKR37ALQTmBhi4zPYppivbe5WK2HG5");

#[program]
pub mod day_3 {
    use super::*;

    pub fn initialize(ctx: Context<Signers>, number: u32) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("You entered : {}", number);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Signers<'a> {
    signer: Signer<'a>,
    another_signer: Signer<'a>
}
