use anchor_lang::prelude::*;

#[program]
pub mod solana_anchor_tutorial {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}