import { PublicKey } from "@solana/web3.js";

export const METADATA_PREFIX = "metadata";
const METADATA_PROGRAM_ID = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

export function getLegacyMetadataPda(mint: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(METADATA_PREFIX),
      new PublicKey(METADATA_PROGRAM_ID).toBuffer(),
      mint.toBuffer(),
    ],
    new PublicKey(METADATA_PROGRAM_ID)
  );
}
