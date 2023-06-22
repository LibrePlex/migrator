
use anchor_lang::accounts::signer;
use anchor_lang::{prelude::*};
use anchor_spl::token::Mint;
use libreplex_metadata::cpi::accounts::GroupAdd;
use libreplex_metadata::{Group, CreateMetadataInput, Asset};
use libreplex_metadata::program::LibreplexMetadata as MetadataProgram;
use crate::libreplex_migrator;
use crate::program::LibreplexMigrator as LibrePlexMigrator;
use libreplex_metadata::cpi::accounts::CreateMetadata;
use mpl_token_metadata::state::{Metadata as LegacyMetadata, TokenMetadataAccount};

#[derive(Accounts)]
pub struct MigrateLite<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub group: Option<Account<'info, Group>>,


    #[account()]
    pub mint: Account<'info, Mint>,

    /// CHECK: validated in logic
    #[account()]
    pub legacy_metadata: UncheckedAccount<'info>,

    /// CHECK: validated via CPI
    #[account(mut)]
    pub libreplex_metadata: UncheckedAccount<'info>,

    pub libreplex_metadata_program: Program<'info, MetadataProgram>,

    pub libreplex_migrator_program: Program<'info, LibrePlexMigrator>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<MigrateLite>
) -> Result<()> {
    
    let libreplex_metadata = &ctx.accounts.libreplex_metadata_program;
    let legacy_metadata = &ctx.accounts.legacy_metadata;
    let mint = &ctx.accounts.mint;
    let payer = &ctx.accounts.payer;
    let group = &ctx.accounts.group;
    let system_program = &ctx.accounts.system_program;
    let libreplex_migrator = &ctx.accounts.libreplex_migrator_program;

    let legacy_metadata_obj = LegacyMetadata::from_account_info(
        &legacy_metadata.to_account_info())?;

    if mint.key() != legacy_metadata_obj.mint.key() {
        return Err(ErrorCode::ConstraintTokenMint.into());
    }

    if payer.key() != legacy_metadata_obj.update_authority.key() {
        return Err(ErrorCode::ConstraintMintMintAuthority.into());
    }

    let mint_key = mint.key();

    let signer_seeds = [
        mint_key.as_ref()
    ];
    /*
     create libre metadata, replicating what we can from legacy: name, symbol etc
     for asset, we use Asset::Json as this provides backwards compatible data 
     structure. We do not have to worry about the other asset types here as
     it's completely possible to switch to, say, inscriptions later if we want to.
    */
    libreplex_metadata::cpi::create_metadata(
        CpiContext::new_with_signer(
            libreplex_metadata.to_account_info(),
            CreateMetadata {
                metadata: legacy_metadata.to_account_info(),
                mint: mint.to_account_info(),
                authority: payer.to_account_info(),
                system_program: system_program.to_account_info(),
                signer: payer.to_account_info(),
                invoked_migrator_program: Some(libreplex_migrator.to_account_info())
            },
            &[&signer_seeds]
        ),
        CreateMetadataInput {
            name: legacy_metadata_obj.data.name,
            symbol: legacy_metadata_obj.data.symbol,
            asset: Asset::Json { url: legacy_metadata_obj.data.uri },
            description: None,
            update_authority: payer.key(),
            
        }
    )?;

    match group {
        Some(x) => {
            libreplex_metadata::cpi::group_add(
                CpiContext::new(
                    libreplex_metadata.to_account_info(),
                    GroupAdd {
                        metadata: libreplex_metadata.to_account_info(),
                        metadata_authority: payer.to_account_info(),
                        group_authority: payer.to_account_info(),
                        delegated_metadata_specific_permissions: None,
                        delegated_group_wide_permissions: None,
                        group: x.to_account_info(),
                        system_program: system_program.to_account_info()
                    }
                )
            )?;
        }, None =>{
            // no group supplied. Do not add to group. Doh.
        }
    }
    

    Ok(())
}