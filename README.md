# tx-util

**Utility for rlp-encoding and signing new EIP-2718 typed transactions for testing**

Currently supports type `0x2` and type `0x4` transactions.

## How does it work

`tx-util` accepts json formatted transactions. A typical [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) transaction is defined as:

```json
{
  "chainId": 1,
  "nonce": 10,
  "maxPriorityFeePerGas": 373223425,
  "maxFeePerGas": 34714654540,
  "gasLimit": 63221,
  "destination": "0x695461EF560Fa4d3a3e7332c9bfcEC261c11a1B6",
  "amount": 0,
  "data": "0x",
  "accessList": [
      {
          "address": "0x8DfDf61F2Eb938b207c228b01a2918b196992ABf",
          "storageKeys": [
              "0x0000000000000000000000000000000000000000000000000000000000000003"
          ]
      }
  ]
}
```

A signed EIP-1559 tx can be created from this:

```shell
cat eip1559_tx_file | tx-util encode-tx --tx-type 2 --signer 0x...
```

### EIP-7702

`tx-util` can also sign [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702) transactions and their authorizations.

```json
{
  "chainId": 1,
  "nonce": 0,
  "maxPriorityFeePerGas": 373223425,
  "maxFeePerGas": 34714654540,
  "gasLimit": 63221,
  "destination": "0x695461EF560Fa4d3a3e7332c9bfcEC261c11a1B6",
  "amount": 0,
  "data": "0x",
  "accessList": [
      {
          "address": "0x8DfDf61F2Eb938b207c228b01a2918b196992ABf",
          "storageKeys": [
              "0x0000000000000000000000000000000000000000000000000000000000000003"
          ]
      }
  ],
  "authorizationList": [
      {
          "chainId": 1,
          "address": "0xD571b8bcd11dF08F0459009Dd1bd664127A431Ee",
          "nonce": 2
      },
      {
          "chainId": 1,
          "address": "0xD571b8bcd11dF08F0459009Dd1bd664127A431Ee",
          "nonce": null
      }
  ]
}
```

The number of `--authorizer` must match the number of items in the `authorization_list`:

```shell
cat eip7702_auth_file | tx-util encode-tx --tx-type 2 \
    --signer 0x... \
    --authorizer 0x... \
    --authorizer 0x...
```

## Installation

Installation requires the [rust toolchain](https://rustup.rs/):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`tx-util` can be installed with `cargo`:

```shell
cd /path/to/repo
cargo install --path .
```

#### Uninstall

```shell
cargo uninstall tx-util
```

## Disclaimer

**This utility has not been checked for correctness and should not be used to generate transactions for mainnet. It is for testing purposes only.**

## Getting help

Issues and PRs are welcome. We can also be reached at `gm@otim.xyz` if you have any additional questions.
