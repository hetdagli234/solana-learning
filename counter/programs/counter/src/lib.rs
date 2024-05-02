use anchor_lang::prelude::*;

declare_id!("4gdRvoPjjwbkMtDfWzUcrDghfQTxoV4oU6L2ELz88qUW");

#[program]
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, counter: u8) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = counter;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct CounterAccount {
    pub counter: u8,
}