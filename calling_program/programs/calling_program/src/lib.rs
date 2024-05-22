use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("J2vJYgLFwppyahb2gMrJWJ7Uyn4sq1mCU8q6oJPWP3S7");

#[program]
pub mod program_a {
    use super::*;

    pub fn call_increment(ctx: Context<CallIncrement>) -> Result<()> {
        let cpi_program = ctx.accounts.program_b.to_account_info();
        let cpi_accounts = Increment {
            counter: ctx.accounts.counter.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        program_b::cpi::increment(cpi_context)?;

        msg!("Called Program B to increment counter");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CallIncrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub program_b: Program<'info, ProgramB>,
}

#[account]
pub struct Counter {
    pub count: u64,
}