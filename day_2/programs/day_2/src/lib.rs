use anchor_lang::prelude::*;

declare_id!("6B6JxedhX8kBXHShAjshCTeGuZeEWvzrY69A5wYLxv27");

#[program]
pub mod day_2 {
    use std::{u32::MAX as u32_MAX};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a:u64, b:u64, message: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("You sent : {} and {}", a, b);
        msg!("You said : {}", message);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array is : {:?}", arr);
        Ok(())
    }

    pub fn this_will_overflow(_ctx: Context<Initialize>, a: u32) -> Result<()> {
        let _ = u32_MAX + a;
        Ok(())
    }

    pub fn this_will_panic(_ctx: Context<Initialize>, a: u32) -> Result<()> {
        let _ = u32_MAX.checked_add(a).unwrap();
        Ok(())
    }

    pub fn calculator(_ctx: Context<Initialize>, a: u64, operator: String, b: u64) -> Result<u64> {
        match operator {
            ref op if op == "+" => Ok(a + b),
            ref op if op == "-" => Ok(a - b),
            ref op if op == "/" => Ok(a / b),
            ref op if op == "*" => Ok(a * b),
            ref op if op == "sqrt" => Ok(((a as f64).sqrt()) as u64),
            ref op if op == "log" => Ok(((a as f64).log10()) as u64),
            _ => Err(ErrorCode::InvalidOperator.into())
        }
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid operator")]
    InvalidOperator,
}

#[derive(Accounts)]
pub struct Initialize {}
