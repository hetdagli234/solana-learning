use anchor_lang::prelude::*;

declare_id!("G81nMzE8hsnG9tUqtpGU3XhcF97qc3DKan2iUDLuBWpG");

#[program]
pub mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.counter += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"counter", user.key().as_ref()], bump)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", user.key().as_ref()], bump)]
    pub data_account: Account<'info, DataAccount>,
    pub user: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub counter: u64,
}