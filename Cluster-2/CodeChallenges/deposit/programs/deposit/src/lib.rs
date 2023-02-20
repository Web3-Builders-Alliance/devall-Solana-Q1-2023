use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::*;
use anchor_lang::solana_program::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_into_pda(ctx: Context<DepositInto>, amount_to_pda: u64) -> Result<()> {
        invoke(&system_instruction::transfer(), &[]);

        Ok(())
    }

    pub fn withraw(ctx: Context<DepositInto>, amount_to_pda: u64) -> Result<()> {
        invoke(&system_instruction::transfer(), &[]);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8,
        seeds = [b"deposit".as_ref()],
        bump
    )]
    pub pda: Account<'info, DepositSpace>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositInto<'info> {
    #[account(mut)]
    pub pda: Account<'info, DepositSpace>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositSpace {}
