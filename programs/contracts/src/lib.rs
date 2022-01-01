use anchor_lang::prelude::*;
use percentage::Percentage;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod splits_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, addresses: Vec<Pubkey>, percentages: Vec<u64>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let mut total_percentage = 0;
        let mut index = 0;

        for ac_address in addresses.iter() {
            total_percentage = total_percentage + percentages[index];
            index = index + 1;
        }

        assert_eq!(total_percentage, 100, "INIT:: total percentage should be 100");

        base_account.authority = authority;
        base_account.addresses = addresses;
        base_account.percentages = percentages;

        Ok(())
    }

    pub fn send_sol<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>, amount: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let msg_sender = &mut ctx.accounts.msg_sender;
        let mut index = 0;

        for ac_address in base_account.addresses.iter() {
            let split_percentage = Percentage::from(base_account.percentages[index]);
            let split_amount = split_percentage.apply_to(amount);

            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &msg_sender.key(),
                &ac_address,
                split_amount,
            );
            
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    msg_sender.to_account_info(),
                    ctx.remaining_accounts[index].to_account_info(),
                ],
            );

            index = index + 1;
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

#[account]
pub struct BaseAccount {
    pub authority: Pubkey,
    pub addresses: Vec<Pubkey>,
    pub percentages: Vec<u64>
}

#[derive(Accounts)]
pub struct SenderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReaderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub msg_sender: Signer<'info>,
}