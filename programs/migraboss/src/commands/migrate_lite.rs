use anchor_client::{
    solana_client::rpc_config::RpcSendTransactionConfig,
    solana_sdk::{signature::Keypair, signature::Signer, system_program},
    Client,
};
use anchor_lang::prelude::*;
use anyhow::Result;

use std::rc::Rc;

pub fn migrate_lite(
    program_id: Pubkey,
    client: Client<Rc<Keypair>>,
    payer: &Keypair,
    price: Pubkey,
    atom_index: u8,
    group_idx: u8,
    amount: u64,
    price_type: u8,
) -> Result<()> {
    let program_client = client.program(program_id);

    let (flexipay_pda, _) = Pubkey::find_program_address(
        &[b"flexipay".as_ref(), payer.pubkey().as_ref()],
        &program_id,
    );

    let (atom_pda, bump) = Pubkey::find_program_address(
        &[
            b"atom".as_ref(),
            price.key().as_ref(),
            &atom_index.to_le_bytes()
        ],
        &program_id,
    );
    println!("{:}", atom_pda);
    
    println!("flexipay address {:}", flexipay_pda);
    println!("price address {:}", price);
    
    program_client
        .request()
        .accounts(migrate_lite::accounts::MigrateLite {
            owner: payer.pubkey(),
            flexipay: flexipay_pda,
            price,
            atom: atom_pda,
            system_program: system_program::id(),
        })
        .args(flexipay::instruction::CreateAtomNative {
            group_idx,
            amount,
            bump,
        })
        .signer(payer)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: None,
            encoding: None,
            max_retries: None,
            min_context_slot: None,
        })?;

    Ok(())
}
