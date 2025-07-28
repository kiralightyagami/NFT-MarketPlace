use anchor_lang::prelude::*;


declare_id!();

mod instructions;
mod states;


pub use instructions::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitMarketplace>, fee_percentage: u8) -> Result<()> {
            ctx.accounts.init_marketplace(fee_percentage, ctx.bumps)?;
        Ok(())
    }
}


