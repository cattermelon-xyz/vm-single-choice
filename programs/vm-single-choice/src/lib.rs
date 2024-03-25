use anchor_lang::prelude::*;

declare_id!("2noSBsEGTG9CY2cdCj7Z6oPj8L62fr4yq8pMPPagRdCY");

#[program]
pub mod vm_single_choice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
