use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file, Keypair},
    },
    Client, Cluster,
};
use anchor_lang::prelude::*;
use anyhow::{Result, Ok};
use clap::{Parser, Subcommand};

use std::rc::Rc;
use std::str::FromStr;

use crate::commands::{migrate_lite};

#[derive(Default, Debug, Parser)]
#[clap(author, version, about)]
pub struct ConfigOverride {
    /// Cluster override.
    #[clap(global = true, long = "provider.cluster")]
    pub cluster: Option<Cluster>,
    // /// Wallet override.
    #[clap(global = true, long = "provider.wallet")]
    pub wallet: Option<String>,
    #[clap(
        global = true,
        long = "program-id",
        default_value = "migr1m1An7f3X75nKuuUn9mm3844miK62ZohpqRfQHp"
    )]
    pub program_id: String,
}

#[derive(Debug, Subcommand)]
#[clap(author, version, about)]
pub enum Command {
    MigrateLite {
        payer: Pubkey,
        mint: Pubkey,
        group: Pubkey
    },
}


#[derive(Debug, Parser)]
pub struct Opts {
    #[clap(flatten)]
    pub cfg_override: ConfigOverride,
    #[clap(subcommand)]
    pub command: Command,
}

pub fn entry(opts: Opts) -> Result<()> {
    let wallet = match opts.cfg_override.wallet {
        Some(wallet) => wallet,
        None => shellexpand::tilde("~/.config/solana/id.json").to_string(),
    };

    // Client setup
    let payer = read_keypair_file(wallet.clone()).expect("Example requires a keypair file");
    let payer2 = read_keypair_file(wallet.clone()).expect("Example requires a keypair file");

    let url = match opts.cfg_override.cluster {
        Some(cluster) => cluster,
        None => Cluster::Custom(
            "http://localhost:8899".to_string(),
            "ws://127.0.0.1:8900".to_string(),
        ),
    };

    let program_id: Pubkey = FromStr::from_str(&opts.cfg_override.program_id)?;
    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    match opts.command {
        Command::MigrateLite { 
            payer,
            mint,
            group
        } => 
            migrate_lite(program_id, client, &payer2)
    }
}

fn migrate_lite(program_id: Pubkey, client: Client<Rc<Keypair>>, flexipay: &Pubkey) -> Result<()> {
    let flexipay_obj: libreplex_migrator::MigrateLite = client.program(program_id).account(*flexipay)?;
    println!("{:#?}", flexipay_obj);

    Ok(())
}
