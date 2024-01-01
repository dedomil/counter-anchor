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

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
        if counter.count == u64::MAX {
            return Err(error!(CounterError::CannotIncrement));
        }
        counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
        if counter.count == u64::MIN {
            return Err(error!(CounterError::CannotDecrement));
        }
        counter.count -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
      init,
      seeds = [authority.key().as_ref()],
      bump,
      payer = authority,
      space = 48,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
      mut,
      has_one = authority,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(
      mut,
      has_one = authority,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

#[error_code]
pub enum CounterError {
    #[msg("cannot decrement below zero")]
    CannotDecrement,
    #[msg("cannot increment as size limit exceeded")]
    CannotIncrement,
}
