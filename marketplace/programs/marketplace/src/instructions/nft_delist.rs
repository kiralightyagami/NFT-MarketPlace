use anchor_lang::{
    prelude::*,
};
use anchor_spl::{
    token::{transfer_checked, Token, TransferChecked},
    token_interface::{Mint, TokenAccount},
};

use crate::{
    error::MarketplaceError,
    states::{Listing, Marketplace},
};


#[derive(Accounts)]
pub struct NftDelist<'info> {
    pub nft: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"listing".as_ref(), marketplace.key().as_ref(), seller_authority.key().as_ref()],
        bump = listing.bump,
        close = seller_authority,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = nft,
        associated_token::authority = listing,
    )]
    pub listing_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub seller_authority: Signer<'info>,


    #[account(
        mut,
        associated_token::mint = nft,
        associated_token::authority = seller_authority,
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace".as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

   

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> NftDelist<'info> {
   pub fn transfer_nft_back (&mut self) -> Result<()> {
    
    require!(self.listing.is_active && self.listing.seller_authority == self.seller_authority.key(), MarketplaceError::ListingNotActive);
    

    let marketplace = &self.marketplace.key();
    let seller_authority = &self.seller_authority.key();
    
    let listing_seeds: &[&[u8]] = &[b"listing".as_ref(), marketplace.as_ref(), seller_authority.as_ref(), &[self.listing.bump]];
    let signer = &[listing_seeds];
    
    let cpi_ctx = CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        TransferChecked {
            from: self.listing_token_account.to_account_info(),
            to: self.seller_token_account.to_account_info(),
            mint: self.nft.to_account_info(),
            authority: self.listing.to_account_info(),
        },
        signer,
    );

    transfer_checked(cpi_ctx, 1,0)?;
    Ok(())
   }

}
