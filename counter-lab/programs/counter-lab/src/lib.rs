use anchor_lang::prelude::*;

declare_id!("At9ckAgMM2QJpBzjwBZMcDew72xwQTw6cfFgMMREJTAr");

#[program]
pub mod counter_lab {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account.counter;
        *counter = 0;
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account.counter;
        *counter +=1;
        Ok(())
    }

    pub fn decrement_counter(ctx: Context<DecrementCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account.counter;
        *counter -=1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = user, space = 8+8)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct DecrementCounter<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>
}

#[account]
pub struct CounterAccount{
    counter: u8
}