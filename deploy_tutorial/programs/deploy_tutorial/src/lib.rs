use anchor_lang::prelude::*;

declare_id!("6pWdiyhjDP2BpU49fcZgS8aF9db3YGQq9eJSWZykwB1P");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
