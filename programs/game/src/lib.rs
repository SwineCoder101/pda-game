use anchor_lang::prelude::*;

declare_id!("9HcGgyHiZhAWRGWfnw9ay9A1B7gxsQDBfBXFbJ92Zc2M");

#[program]
pub mod game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
