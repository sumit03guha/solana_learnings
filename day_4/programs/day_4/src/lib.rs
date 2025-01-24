use anchor_lang::prelude::*;

declare_id!("EabHrNd6S9eftdasCnAiz4zKosGWZcS98upxr5o5xqCf");

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_range(ctx: Context<Initialize>, number: u32) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        require!(number < 10, NumberError::NumberGreaterThan10);
        require!(number > 3, NumberError::NumberLesserThan3);
        Ok(())
    }

    pub fn this_function_always_reverts(_ctx: Context<Initialize>) -> Result<()> {
        msg!("will this print?");
        return err!(MyError::AlwaysReverts)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum NumberError {
    #[msg("number is not less than 10")]
    NumberGreaterThan10,
    #[msg("number is not greater than 3")]
    NumberLesserThan3
}

#[error_code]
pub enum MyError {
    #[msg("This always reverts")]
    AlwaysReverts
}
