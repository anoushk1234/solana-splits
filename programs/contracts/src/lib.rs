use anchor_lang::prelude::*;
use percentage::Percentage;
use anchor_lang::prelude::Pubkey;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod splits_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        if ctx.accounts.base_account.inited == false {
            let base_account = &mut ctx.accounts.base_account;
            base_account.total_amount = 0;
            base_account.total_splits = 0;
            base_account.inited = true;
            base_account.authority = base_account.key();
        }

        Ok(())
    }

    pub fn new_split(ctx: Context<NewSplitContext>, addresses: Vec<Pubkey>, percentages: Vec<u64>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let admin_account = &mut ctx.accounts.admin_account;
        let mut total_percentage = 0;
        let mut index = 0;

        for ac_address in addresses.iter() {
            total_percentage = total_percentage + percentages[index];
            index = index + 1;
        }

        assert_eq!(total_percentage, 100, "NEW SPLIT: total percentage should be 100");

        admin_account.admin_address = admin_account.key();
        admin_account.addresses = addresses;
        admin_account.percentages = percentages;
        base_account.total_splits = base_account.total_splits + 1;

        Ok(())
    }

    pub fn send_sol<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, SenderContext<'info>>, amount: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let admin_account = &mut ctx.accounts.admin_account;
        let msg_sender = &mut ctx.accounts.msg_sender;
        let mut index = 0;

        // iterate over the remaining_accounts
        // check if the account pubkey exists in list of pubkeys
        // find the index of pubkey
        // fetch the percentage with the same index in list of percentages

        for rc_account in ctx.remaining_accounts.iter() {
            if admin_account.addresses.contains(&rc_account.key()) {
                let pubkey_index = admin_account.addresses.iter().position(|&r| r == rc_account.key()).unwrap();
                let split_percentage = Percentage::from(admin_account.percentages[pubkey_index]);
                let split_amount = split_percentage.apply_to(amount);

                let ix = anchor_lang::solana_program::system_instruction::transfer(
                    &msg_sender.key(),
                    &rc_account.key(),
                    split_amount
                );

                anchor_lang::solana_program::program::invoke(
                    &ix,
                    &[
                        msg_sender.to_account_info(),
                        ctx.remaining_accounts[index].to_account_info()
                    ]
                );
                
                index = index + 1;
            }
        }

        base_account.total_amount = amount;

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

#[derive(Accounts)]
pub struct NewSplitContext<'info> {
    #[account(init, payer = user, space = 6942)]
    pub admin_account: Account<'info, AdminAccount>,
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct BaseAccount {
    pub inited: bool,
    pub authority: Pubkey,
    pub total_splits: u64,
    pub total_amount: u64
}

#[account]
pub struct AdminAccount {
    pub admin_address: Pubkey,
    pub addresses: Vec<Pubkey>,
    pub percentages: Vec<u64>
}

#[derive(Accounts)]
pub struct SenderContext<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    pub admin_account: Account<'info, AdminAccount>,
    pub msg_sender: Signer<'info>,
}