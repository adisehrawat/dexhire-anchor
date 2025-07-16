/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/dexhire.json`.
 */
export type Dexhire = {
  "address": "53AaBgyUc8W2DbT2qEHYVdL2z7c9CGabF139WAeiTRKz",
  "metadata": {
    "name": "dexhire",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createClientProfile",
      "discriminator": [
        55,
        240,
        223,
        154,
        45,
        177,
        34,
        50
      ],
      "accounts": [
        {
          "name": "clientprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  105,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "email",
          "type": "string"
        }
      ]
    },
    {
      "name": "createFreelanceProfile",
      "discriminator": [
        232,
        48,
        189,
        223,
        100,
        44,
        147,
        183
      ],
      "accounts": [
        {
          "name": "freelanceprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  114,
                  101,
                  101,
                  108,
                  97,
                  110,
                  99,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "email",
          "type": "string"
        }
      ]
    },
    {
      "name": "deleteClientProfile",
      "discriminator": [
        47,
        223,
        132,
        131,
        17,
        153,
        190,
        86
      ],
      "accounts": [
        {
          "name": "clientprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  105,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "deleteFreelanceProfile",
      "discriminator": [
        211,
        63,
        194,
        107,
        58,
        11,
        232,
        255
      ],
      "accounts": [
        {
          "name": "freelanceprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  114,
                  101,
                  101,
                  108,
                  97,
                  110,
                  99,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "updateClientProfile",
      "discriminator": [
        218,
        224,
        108,
        125,
        81,
        17,
        244,
        192
      ],
      "accounts": [
        {
          "name": "clientprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  105,
                  101,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "email",
          "type": "string"
        },
        {
          "name": "bio",
          "type": "string"
        },
        {
          "name": "country",
          "type": "string"
        },
        {
          "name": "linkedin",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateFreelanceProfile",
      "discriminator": [
        253,
        121,
        98,
        115,
        13,
        11,
        245,
        203
      ],
      "accounts": [
        {
          "name": "freelanceprofile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  114,
                  101,
                  101,
                  108,
                  97,
                  110,
                  99,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "email",
          "type": "string"
        },
        {
          "name": "bio",
          "type": "string"
        },
        {
          "name": "skills",
          "type": {
            "vec": {
              "defined": {
                "name": "skill"
              }
            }
          }
        },
        {
          "name": "country",
          "type": "string"
        },
        {
          "name": "linkedin",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "clientProfile",
      "discriminator": [
        24,
        67,
        86,
        10,
        117,
        219,
        200,
        237
      ]
    },
    {
      "name": "freelancerProfile",
      "discriminator": [
        142,
        199,
        151,
        44,
        211,
        185,
        36,
        26
      ]
    }
  ],
  "types": [
    {
      "name": "clientProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "email",
            "type": "string"
          },
          {
            "name": "bio",
            "type": "string"
          },
          {
            "name": "country",
            "type": "string"
          },
          {
            "name": "linkedin",
            "type": "string"
          },
          {
            "name": "avatar",
            "type": "string"
          },
          {
            "name": "joinedAt",
            "type": "i64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "freelancerProfile",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "email",
            "type": "string"
          },
          {
            "name": "bio",
            "type": "string"
          },
          {
            "name": "skills",
            "type": {
              "vec": {
                "defined": {
                  "name": "skill"
                }
              }
            }
          },
          {
            "name": "country",
            "type": "string"
          },
          {
            "name": "linkedin",
            "type": "string"
          },
          {
            "name": "avatar",
            "type": "string"
          },
          {
            "name": "joinedAt",
            "type": "i64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "skill",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          }
        ]
      }
    }
  ]
};
