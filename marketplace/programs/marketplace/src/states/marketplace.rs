use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]


pub struct Marketplace {
    pub authority: Pubkey,
    pub bump: u8,
    pub fee_percentage: u8,
    pub treasury_bump: u8,
}
