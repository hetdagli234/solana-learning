use anchor_lang::prelude::*;

declare_id!("EsfZnoKrr3icwNtSxjJFuDsf66eed1KWMDX8mv6NFJcN");

#[program]
pub mod called_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
