use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller_authority: Pubkey,
    pub bump: u8,
    pub item_mint: Pubkey,
    pub item_price: u64,
}