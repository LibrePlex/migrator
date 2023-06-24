
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID_MIGRATOR } from "./getProgramInstance";


export const getLibreSignerPda = (mint: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata_signer"),
      mint.toBuffer()],
    new PublicKey(PROGRAM_ID_MIGRATOR)
  );
};
