use anchor_lang::prelude::*;

declare_id!("9dw8AoNJsdkDvFNP7LCsRW37jpUQLNjUVYs9EozRtRkz");

#[program]
pub mod events {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn this_emits_events(_ctx:Context<Initialize>, a: u32, b: String) -> Result<()> {
        emit!(MyEvent {name: b, age: a});
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub name: String,
    pub age: u32
}
