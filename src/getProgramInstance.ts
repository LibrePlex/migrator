import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair } from "@solana/web3.js";

export const PROGRAM_ID_MIGRATOR = "migr1m1An7f3X75nKuuUn9mm3844miK62ZohpqRfQHp";

import { IDL } from "../target/types/libreplex_migrator";

export function getProgramInstance(
  connection: Connection,
  wallet: anchor.Wallet
) {
  if (!wallet.publicKey) return;
  const provider = new anchor.AnchorProvider(
    connection,
    wallet,
    anchor.AnchorProvider.defaultOptions()
  );
  // Read the generated IDL.
  const idl = IDL;
  // Address of the deployed program.
  const programId = PROGRAM_ID_MIGRATOR;
  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId, provider);
  return program;
}
