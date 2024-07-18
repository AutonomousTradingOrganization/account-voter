use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("6dPHkxoJGDyt4h37htgrm62zJU6aLdfHFcfX7KcHcyae");

#[program]
mod account_voter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, pseudo: [u8; 256], mail: [u8; 256], balance_total: u16, balance_sol: u16, total_trade: u16, total_participation: u16, win_trade: u16) -> Result<()> {
        ctx.accounts.new_account.pseudo = pseudo;
        ctx.accounts.new_account.mail = mail;
        ctx.accounts.new_account.balance_total = balance_total;
        ctx.accounts.new_account.balance_sol = balance_sol;
        ctx.accounts.new_account.total_trade = total_trade;
        ctx.accounts.new_account.total_participation = total_participation;
        ctx.accounts.new_account.win_trade = win_trade;
        msg!("Changed data to: {} {} {} {} {}!", balance_total, balance_sol, total_trade, total_participation, win_trade);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + NewAccount::INIT_SPACE,
        seeds = [b"accountVoter".as_ref(), signer.key().as_ref()],
        bump
    )]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct NewAccount {
    pseudo: [u8; 256],
    mail: [u8; 256],
    balance_total: u16,
    balance_sol: u16,
    total_trade: u16,
    total_participation: u16,
    win_trade: u16,
}
