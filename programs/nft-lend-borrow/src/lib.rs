use anchor_lang::prelude::*;

declare_id!("539Cu2TavqmTpqW6yueREntmM5vetHKDGLsgu2c9FXCG");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
