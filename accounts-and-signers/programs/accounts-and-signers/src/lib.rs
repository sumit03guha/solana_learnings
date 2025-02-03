use anchor_lang::prelude::*;

declare_id!("GgupEyyqv3EXoBjKKXFAPiTzjH3aHBsXcjhAyisUd2q1");

const OWNER: &str = "2cZvfJjkdkzoeVCEZsdtjQWmA46DcTcKD4HgyizcmVZs";

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

    pub fn call_from_multiple_signers(ctx: Context<Initialize>) -> Result<()> {
        let signer = &ctx.accounts.signer;
        let signer2 = &ctx.accounts.signer2;
        let signer3 = &ctx.accounts.signer3;
        msg!("Namaste from : {:?}, {:?}, {:?}", signer, signer2, signer3);
        Ok(())
    }

    #[access_control(only_owner(&ctx))]
    pub fn only_owner_function(ctx: Context<Initialize>) -> Result<()> {
        msg!("I am the owner");
        Ok(())
    }

}

fn only_owner(ctx: &Context<Initialize>) -> Result<()> {
    require_keys_eq!(ctx.accounts.signer.key(), OWNER.parse::<Pubkey>().unwrap(), OwnerError::NotOwner);
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>
}

#[error_code]
pub enum OwnerError {
    #[msg("Only the owner can call this function")]
    NotOwner
}
