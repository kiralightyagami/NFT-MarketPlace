use anchor_lang::prelude::*;
use crate::states::*;
use anchor_spl::token::*;
use anchor_spl::token_interface::*;



#[derive(Accounts)]
pub struct NftList<'info> {
    pub nft: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = seller_authority,
        space = 8 + std::mem::size_of::<Listing>(),
        seeds = [b"listing".as_ref(), marketplace.key().as_ref(), seller_authority.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init,
        payer = seller_authority,
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
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

   

    #[account(
        seeds = [b"treasury".as_ref(), marketplace.key().as_ref()],
        bump,
    )]

    pub treasury: SystemAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> NftList<'info> {
   pub fn transfer_nft (&mut self) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        self.token_program.to_account_info(),
        TransferChecked {
            from: self.seller_token_account.to_account_info(),
            to: self.listing_token_account.to_account_info(),
            mint: self.nft.to_account_info(),
            authority: self.seller_authority.to_account_info(),
        }
    );

    transfer_checked(cpi_ctx, 1,0)?;
   }

   pub fn init_listing (&mut self, bumps: NftListBumps, price: u64) -> Result<()> {
    self.listing.set_inner(Listing {
        seller_authority: self.seller_authority.key(),
        item_mint: self.nft.key(),
        item_price: price,   
        bump: bumps.listing,
    });
    Ok(())
   }
}
