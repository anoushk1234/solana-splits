use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod SplitsProgram {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, splits: Vec<Splits>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut total_percentage = 0;

        for split in splits.iter() {
            total_percentage = total_percentage + split.percentage
        }

        assert_eq!(total_percentage, 100, "INIT:: total percentage should be 100");

        base_account.authority = authority;
        base_account.splits = splits;

        Ok(())
    }

    pub fn pay(ctx: Context<PayerContext>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let msg_sender = ctx.accounts.msg_sender.key();

        for split in base_account.splits.iter() {
            
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 6942)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Splits {
    pub address: Pubkey,
    pub percentage: u64,
}

#[account]
pub struct BaseAccount {
    pub authority: Pubkey,
    pub splits: Vec<Splits>
}

#[derive(Accounts)]
pub struct PayerContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
}