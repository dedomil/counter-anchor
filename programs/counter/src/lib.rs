use anchor_lang::prelude::*;

declare_id!("NotEHXeyyx62JrqCC3t3G79pbXvxfq8jU5GVzBc1uQq");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
        counter.authority = ctx.accounts.authority.key();
        counter.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
      init,
      payer =authority,
      space = 48,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
