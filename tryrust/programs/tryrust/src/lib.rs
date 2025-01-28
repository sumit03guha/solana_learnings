use anchor_lang::prelude::*;

declare_id!("4yEtV8g1oNJEEY3Vw2qbzQ1UpAP4AqCpTBsWRahFmQUx");

#[program]
pub mod tryrust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn age_checker(_ctx: Context<Initialize>, age: u8) -> Result<()> {
        if age < 18 {
            msg!("You are below 18.")
        } else {
            msg!("You are 18 or above.")
        }
        Ok(())
    }

    pub fn check_age_with_match(_ctx: Context<Initialize>, age: u8) -> Result<()> {
        match age {
            0..=17 => msg!("match : You are below 18."),
            _ => msg!("match : You are 18 or above.")
        }
        Ok(())
    }

    pub fn trying_hashmap(_ctx: Context<Initialize>, name: String, age: u8) -> Result<()> {
        use std::collections::HashMap;

        let mut id_hashmap = HashMap::new();

        id_hashmap.insert(&name, age);

        let id_age = id_hashmap.get(&name).unwrap();

        msg!(&format!("The identity is :: name : {}, age : {}", &name, id_age));

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
