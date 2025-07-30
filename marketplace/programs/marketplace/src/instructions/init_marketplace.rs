use anchor_lang::prelude::*;
use crate::states::*;
use crate::error::MarketplaceError;


#[derive(Accounts)]
pub struct InitMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Marketplace>(),
        seeds = [b"marketplace".as_ref()],
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury".as_ref(), marketplace.key().as_ref()],
        bump,
    )]

    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitMarketplace<'info> {
    pub fn init_marketplace(&mut self, fee_percentage: u8, bumps: InitMarketplaceBumps) -> Result<()> {
        require!(fee_percentage <= 100, MarketplaceError::InvalidFeePercentage);
        
        self.marketplace.set_inner(Marketplace {    
            authority: self.authority.key(),
            fee_percentage,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
        });
        Ok(())
    }
}
