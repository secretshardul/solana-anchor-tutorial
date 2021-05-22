/**
 * Tutorial 2: Creating and mutating accounts

 * We will create new accounts owned by the program, where data can be written
 */


use anchor_lang::prelude::*;

#[program]
pub mod t2_accounts_and_args {
    use super::*;

    /**
     * Constructor function

     * Save 'data' into the account
     */
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;

        Ok(())
    }
}

#[account]
pub struct MyAccount {
    pub data: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)] // Use init for newly created program accounts
    pub my_account: ProgramAccount<'info, MyAccount>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)] // Use mut to update existing program accounts
    pub my_account: ProgramAccount<'info, MyAccount>
}
