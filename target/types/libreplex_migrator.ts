export type LibreplexMigrator = {
  "version": "0.4.0",
  "name": "libreplex_migrator",
  "instructions": [
    {
      "name": "migrateLite",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "migrateSigner",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata_signer"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "libreplexMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMigratorProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "migrateSignerBump",
          "type": "u8"
        }
      ]
    }
  ]
};

export const IDL: LibreplexMigrator = {
  "version": "0.4.0",
  "name": "libreplex_migrator",
  "instructions": [
    {
      "name": "migrateLite",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "group",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "migrateSigner",
          "isMut": false,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metadata_signer"
              },
              {
                "kind": "account",
                "type": "publicKey",
                "account": "Mint",
                "path": "mint"
              }
            ]
          }
        },
        {
          "name": "legacyMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "libreplexMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "libreplexMigratorProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "migrateSignerBump",
          "type": "u8"
        }
      ]
    }
  ]
};
