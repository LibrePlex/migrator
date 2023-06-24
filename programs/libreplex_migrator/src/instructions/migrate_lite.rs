use std::str;

use std::borrow::Borrow;

use anchor_lang::accounts::signer;
use anchor_lang::{prelude::*};
use anchor_spl::token::Mint;
use libreplex_metadata::cpi::accounts::GroupAdd;
use libreplex_metadata::{Group, CreateMetadataInput, Asset};
use libreplex_metadata::program::LibreplexMetadata as MetadataProgram;
use crate::libreplex_migrator;
use crate::program::LibreplexMigrator as LibrePlexMigrator;
use libreplex_metadata::cpi::accounts::CreateMetadata;

#[derive(Accounts)]
pub struct MigrateLite<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub group: Option<Account<'info, Group>>,


    #[account()]
    pub mint: Account<'info, Mint>,

    /// CHECK: validated in logic
    #[account(
        seeds=[
            b"metadata_signer",
            mint.key().as_ref()
        ],
        bump
    )]
    pub migrate_signer: UncheckedAccount<'info>,

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
    ctx: Context<MigrateLite>,
    migrate_signer_bump: u8
) -> Result<()> {
    
    let libreplex_metadata = &ctx.accounts.libreplex_metadata;
    let libreplex_metadata_program = &ctx.accounts.libreplex_metadata_program;
    let legacy_metadata = &ctx.accounts.legacy_metadata;
    let mint = &ctx.accounts.mint;
    let payer = &ctx.accounts.payer;
    let migrate_signer = &ctx.accounts.migrate_signer;
    let group = &ctx.accounts.group;
    let system_program = &ctx.accounts.system_program;
    let libreplex_migrator = &ctx.accounts.libreplex_migrator_program;

    
    let a = legacy_metadata.to_account_info().clone();
    let md_accountinfo = &a.data.borrow_mut()[..];
    msg!("Parsing mint id");
    let md_mint = Pubkey::try_from_slice(&md_accountinfo[33..65])?;

    msg!("Parsing uauth id");
    let md_uauth = Pubkey::try_from_slice(&md_accountinfo[1..33])?;

    msg!("Checking integrity");
    if mint.key() != md_mint {
        return Err(ErrorCode::ConstraintTokenMint.into());
    }

    if payer.key() != md_uauth {
        return Err(ErrorCode::ConstraintMintMintAuthority.into());
    }

    let mint_key = mint.key();

    let signer_seeds = [
        b"metadata_signer",
        mint_key.as_ref(),
        &[migrate_signer_bump]
    ];
    

    let name_length = u32::try_from_slice(&md_accountinfo[65..69])? as usize;
    msg!("Name length {}", name_length);
    let name = str::from_utf8(&md_accountinfo[69..(69+name_length)]).unwrap().trim_end_matches(char::from(0)).to_string();
    msg!("Name {}", name);

    let symbol_length = u32::try_from_slice(&md_accountinfo[101..105])? as usize;
    msg!("Symbol length {}", symbol_length);
    let symbol = str::from_utf8(&md_accountinfo[105..(105+symbol_length)]).unwrap().trim_end_matches(char::from(0)).to_string();
    msg!("Symbol {}", symbol);

    let uri_length = u32::try_from_slice(&md_accountinfo[115..119])? as usize;
    msg!("URL length {}", uri_length);
    let url = str::from_utf8(&md_accountinfo[119..(119+uri_length)]).unwrap().trim_end_matches(char::from(0)).to_string();
    msg!("Url {}", url);


    /*
     create libre metadata, replicating what we can from legacy: name, symbol etc
     for asset, we use Asset::Json as this provides backwards compatible data 
     structure. We do not have to worry about the other asset types here as
     it's completely possible to switch to, say, inscriptions later if we want to.
    */
    libreplex_metadata::cpi::create_metadata(
        CpiContext::new_with_signer(
            libreplex_metadata_program.to_account_info(),
            CreateMetadata {
                metadata: libreplex_metadata.to_account_info(),
                mint: mint.to_account_info(),
                authority: migrate_signer.to_account_info(),
                system_program: system_program.to_account_info(),
                signer: payer.to_account_info(),
                invoked_migrator_program: Some(libreplex_migrator.to_account_info())
            },
            &[&signer_seeds]
        ),
        CreateMetadataInput {
            name,
            symbol,
            asset: Asset::Json { url: url.to_owned()}, //legacy_metadata_obj.data.uri },
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
                        metadata_authority: payer.to_account_info(),
                        group_authority: payer.to_account_info(),
                        metadata: libreplex_metadata.to_account_info(),
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

// pub fn assert_pda_derivation(program_id: &Pubkey, account_key: &Pubkey, path: &[&[u8]]) -> Result<u8> {
//     let (key, bump) = Pubkey::find_program_address(path, program_id);
//     if key != *account_key {
//         return err!(ErrorCode::DerivedKeyInvalid);
//     }
//     Ok(bump)
// }