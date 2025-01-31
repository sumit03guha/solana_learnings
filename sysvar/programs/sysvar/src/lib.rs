use anchor_lang::prelude::*;

declare_id!("aCAT59XoL7QCDCLCE5LmQyeK6dauLtPMDKamt7cSRS5");

#[program]
pub mod sysvar {
    use super::*;
    use chrono::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn clock(_ctx:Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        msg!("Block timestamp : {:?}", clock);
        Ok(())
    }

    pub fn clock_with_chrono(_ctx:Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;
        let date_time = chrono::DateTime::from_timestamp(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!("Week day is : {}", day_of_the_week);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
