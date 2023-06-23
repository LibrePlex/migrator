export type LibreplexMigrator = {
  "version": "0.3.0",
  "name": "libreplex_migrator",
  "instructions": [
    {
      "name": "canonical",
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
      "args": []
    }
  ]
};

export const IDL: LibreplexMigrator = {
  "version": "0.3.0",
  "name": "libreplex_migrator",
  "instructions": [
    {
      "name": "canonical",
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
      "args": []
    }
  ]
};
