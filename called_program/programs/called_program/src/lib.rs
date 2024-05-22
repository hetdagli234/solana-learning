use anchor_lang::prelude::*;

declare_id!("EsfZnoKrr3icwNtSxjJFuDsf66eed1KWMDX8mv6NFJcN");

#[program]
pub mod called_program {
    use super::*;

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented to: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
