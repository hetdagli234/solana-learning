use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("HgAdgVTnink65ufJ9Y8YbBbMku7ttkKvPEUKX4SPpaVJ");

#[program]
pub mod create_pda_example {
    use super::*;

    pub fn create_account_cpi(ctx: Context<CreatePdaAccount>) -> Result<()> {

        let lamports = Rent::get()?.minimum_balance(8);

        anchor_lang::system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.pda_account.to_account_info(),
                },
            ),
            lamports,
            8,
            ctx.program_id,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePdaAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub pda_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}