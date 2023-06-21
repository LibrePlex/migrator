
use anchor_lang::{prelude::*};
use anchor_spl::token::Mint;
use libreplex_metadata::cpi::accounts::GroupAdd;
use libreplex_metadata::{Group, CreateMetadataInput, Asset};
use libreplex_metadata::program::LibreplexMetadata as MetadataProgram;
use libreplex_metadata::cpi::accounts::CreateMetadata;
use mpl_token_metadata::state::{Metadata as LegacyMetadata, TokenMetadataAccount};

#[repr(C)]
#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct MigrateLiteInput {
    // name, symbol and creators are picked up 
    // from legacy metadata. however the remaining fields
    // must be specified if they are required on chain
    attributes: Vec<u8>, // group to add the migrated metadata to

}

#[derive(Accounts)]
#[instruction(ordinal_input: MigrateLiteInput)]
pub struct MigrateLite<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub root: Signer<'info>,

    #[account(mut)]
    pub group: Account<'info, Group>,

    #[account()]
    pub mint: Account<'info, Mint>,

    /// CHECK: validated in logic
    #[account()]
    pub legacy_metadata: UncheckedAccount<'info>,

    /// CHECK: validated via CPI
    #[account(mut)]
    pub libreplex_metadata: UncheckedAccount<'info>,

    pub libreplex_metadata_program: Program<'info, MetadataProgram>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<MigrateLite>,
    inscription_input: MigrateLiteInput,
) -> Result<()> {
    
    let libreplex_metadata = &ctx.accounts.libreplex_metadata_program;
    let legacy_metadata = &ctx.accounts.legacy_metadata;
    let mint = &ctx.accounts.mint;
    let payer = &ctx.accounts.payer;
    let group = &ctx.accounts.group;
    let system_program = &ctx.accounts.system_program;

    let legacy_metadata_obj = LegacyMetadata::from_account_info(
        &legacy_metadata.to_account_info())?;

    if mint.key() != legacy_metadata_obj.mint.key() {
        return Err(ErrorCode::ConstraintTokenMint.into());
    }

    if payer.key() != legacy_metadata_obj.update_authority.key() {
        return Err(ErrorCode::ConstraintMintMintAuthority.into());
    }

    libreplex_metadata::cpi::create_metadata(
        CpiContext::new(
            libreplex_metadata.to_account_info(),
            CreateMetadata {
                // raffle is the owner of the pod
                metadata: legacy_metadata.to_account_info(),
                mint: mint.to_account_info(),
                system_program: system_program.to_account_info(),
                signer: payer.to_account_info()
            }
        ),
        CreateMetadataInput {
            name: legacy_metadata_obj.data.name,
            symbol: legacy_metadata_obj.data.symbol,
            asset: Asset::Json { url: legacy_metadata_obj.data.uri },
            description: None,
            update_authority: payer.key()
        }
    )?;

    libreplex_metadata::cpi::group_add(
        CpiContext::new(
            libreplex_metadata.to_account_info(),
            GroupAdd {
                metadata: libreplex_metadata.to_account_info(),
                metadata_authority: payer.to_account_info(),
                group_authority: payer.to_account_info(),
                delegated_metadata_specific_permissions: None,
                delegated_group_wide_permissions: None,
                group: group.to_account_info(),
                system_program: system_program.to_account_info()
            }
        )
    )?;

    Ok(())
}