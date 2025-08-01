use anchor_lang::prelude::*;


declare_id!()

pub mod instructions;
pub mod states;
pub mod error;
pub mod constants;

pub use instructions::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<InitMarketplace>, fee_percentage: u8) -> Result<()> {
            ctx.accounts.init_marketplace(fee_percentage, ctx.bumps)?;
        Ok(())
    }

    pub fn nft_list(ctx: Context<NftList>, price: u64) -> Result<()> {
        ctx.accounts.init_listing(ctx.bumps, price)?;
        ctx.accounts.transfer_nft()?;
        Ok(())
    }

    pub fn delist_nft(ctx: Context<NftDelist>) -> Result<()> {
        ctx.accounts.transfer_nft_back()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<NftBuy>) -> Result<()> {
        ctx.accounts.transfer_nft()?;
        ctx.accounts.transfer_sol()?;
        ctx.accounts.delist_nft()?;
        Ok(())
    }
}


