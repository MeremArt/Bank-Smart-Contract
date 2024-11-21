use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
}