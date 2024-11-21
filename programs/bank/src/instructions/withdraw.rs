use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
}