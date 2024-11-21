use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}