use anchor_lang::prelude::*;

declare_id!("C9kyEYdAojtoAZD5yt9NCYMTWf6x2mnQKZqA7PXLnyzH");

#[program]
pub mod direct_blink {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
