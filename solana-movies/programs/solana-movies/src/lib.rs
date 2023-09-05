use anchor_lang::prelude::*;

declare_id!("G4qKer2mVrNTSEBKEvMMdhjZ2JmdNYyonDkpHeWiiJGX");

#[program]
pub mod solana_movies {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
