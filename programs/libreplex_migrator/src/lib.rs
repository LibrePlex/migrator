use anchor_lang::prelude::*;
use instructions::*;


use anchor_lang::{AnchorDeserialize, AnchorSerialize};



pub mod instructions;



declare_id!("migr1m1An7f3X75nKuuUn9mm3844miK62ZohpqRfQHp");


#[program]
pub mod libreplex_migrator {





    use super::*;
    pub fn canonical(
        ctx: Context<MigrateLite>
    ) -> Result<()> {
        instructions::migrate_lite::handler(ctx)
    }


}
