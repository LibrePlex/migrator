
import { PublicKey } from "@solana/web3.js";


export const PROGRAM_ID_METADATA =
  "LibrQsXf9V1DmTtJLkEghoaF1kjJcAzWiEGoJn8mz7p";

export const getLibreMetadataPda = (mint: PublicKey) => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), mint.toBuffer()],
    new PublicKey(PROGRAM_ID_METADATA)
  );
};
