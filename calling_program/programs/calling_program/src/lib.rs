use anchor_lang::prelude::*;

declare_id!("J2vJYgLFwppyahb2gMrJWJ7Uyn4sq1mCU8q6oJPWP3S7");

#[program]
pub mod calling_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
