const dotenv = require("dotenv");
dotenv.config({ path: `.env.${process.env.NODE_ENV}` });
import { Keypair } from "@solana/web3.js";
import {
  Connection,
  Transaction,
  PublicKey,
  SystemProgram,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { PROGRAM_ID_MIGRATOR, getProgramInstance } from "./getProgramInstance";
import fs from "fs";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { getLegacyMetadataPda } from "./getLegacyMetadataPda";
import {
  PROGRAM_ID_METADATA,
  getLibreMetadataPda,
} from "./getLibreMetadataPda";
import { getLibreSignerPda } from "./getLibreSignerPda";

const { program } = require("commander");

program
  .name("migrator")
  .description("CLI to some JavaScript string utilities")
  .version("0.1.0");

program
  .command("migrate")
  .description("Creates base libre metadata for a mint")
  .argument("<mintId>", "mintId to migrate")
  .option(
    "-k, --keypair <keypair>",
    "keypair to use, defaults to ~/.config/solana/id.json",
    "~/.config/solana/id.json"
  )
  .requiredOption("-g, --group <group>", "group to add migrated metadata to")
  .action(
    async (
      mintId: string,
      options: { keypair: string | undefined; group: string; mint: string }
    ) => {
      const keyfile = JSON.parse(fs.readFileSync(options.keypair, "utf8"));

      const updateAuthKeypair = Keypair.fromSecretKey(new Uint8Array(keyfile));

      const connection = new Connection(process.env.RPC_ENDPOINT);

      const wallet = new NodeWallet(Keypair.generate());

      const program = getProgramInstance(connection, wallet);

      const { group } = options;
      const mint = new PublicKey(mintId);

      const [libreMetadata] = getLibreMetadataPda(mint);
      const [legacyMetadata] = getLegacyMetadataPda(mint);
      const [migrateSigner, migrateSignerBump] = getLibreSignerPda(mint);
      console.log({
        legacyMetadata: legacyMetadata.toBase58(),
        libreMetadata: libreMetadata.toBase58(),
      });

      const instruction = await program.methods
        .migrateLite(
          migrateSignerBump
        )
        .accounts({
          payer: updateAuthKeypair.publicKey,
          group: new PublicKey(group),
          mint,
          legacyMetadata,
          migrateSigner,
          libreplexMetadata: libreMetadata,
          libreplexMetadataProgram: new PublicKey(PROGRAM_ID_METADATA),
          libreplexMigratorProgram: new PublicKey(PROGRAM_ID_MIGRATOR),
          systemProgram: SystemProgram.programId,
        })
        .instruction();

      const transaction = new Transaction().add(instruction);
      transaction.feePayer = updateAuthKeypair.publicKey;

      await sendAndConfirmTransaction(
        connection,
        transaction,
        [updateAuthKeypair],
        {
          skipPreflight: true,
        }
      );
    }
  );

program.parse();
