use anchor_lang::prelude::*;

// Your program Id will be added here when you enter "build" command
declare_id!("");

#[program]
pub mod pdamanager {

    use super::*;

    pub fn initialize_pda(
        ctx: Context<Initializepda>,
        id: u64,
        merchant_name: String,
        amount: u64,
    ) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;

        pda_account.id = id;
        pda_account.merchant_name = merchant_name;
        pda_account.amount = amount;
        pda_account.owner = *ctx.accounts.authority.key;

        Ok(())
    }

    pub fn modify_pda(
        ctx: Context<Modifypda>,
        _id: u64,
        merchant_name: String,
        amount: u64,
    ) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.merchant_name = merchant_name;
        pda_account.amount = amount;

        Ok(())
    }

    pub fn delete_pda(_ctx: Context<Deletepda>, _id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct Initializepda<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32+ (4 + 12)+ 8 + 1,
        seeds = [b"pda", authority.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, pdaAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct Modifypda<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pda", authority.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, pdaAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id : u64)]
pub struct Deletepda<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [b"pda", authority.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, pdaAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct pdaAccount {
    pub id: u64,
    pub owner: Pubkey,
    pub merchant_name: String,
    pub amount: u64,
}
