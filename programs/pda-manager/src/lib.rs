use anchor_lang::prelude::*;

declare_id!("sV6jc9DFyyceTiDazEzr7jHYXvzfsktke9UmtXTPydi");

#[program]
pub mod pda_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(instruction)]
pub struct Initialize {}

struct ConversationPda<'info> {
    #[account (
    seeds = [b"conversation_pda"],
    bump
    )]


}


