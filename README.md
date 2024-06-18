# tx-util

**Utility for rlp-encoding and signing new EIP-2718 typed transactions for testing**

## How does it work

`tx-util` accepts transactions in the following format. A typical [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) transaction can be defined as below, representing `[chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas_limit, destination, amount, data, access_list]`

```
[
  0x539
  0x1
  0x163ef001
  0x81527974c
  0xf6f5
  0x9530901da626663c0679615aa345d88f059c2919
  0x
  0xd09de08b
  []
]
```

A signed EIP-1559 tx can be created from this:

```shell
cat eip1559_tx_file | tx-util encode-tx --tx-type 2 | tx-util sign-tx --private-key 0x...
```

### EIP-7702

`tx-util` can also sign arbitrary payloads, e.g., for [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702) authorizations.

```
[
  0x539
  0xdac17f958d2ee523a2206206994597c13d831ec7
  []
]
```

Represents `[chain_id, address, [nonce]]` which can be encoded and signed (with magic byte `0x05`):

```shell
cat eip7702_auth_file | tx-util encode-rlp | tx-util sign-auth --magic 5 --private-key 0x... | tx-util decode-rlp
```

Which will produce e.g.,:

```
[
  0x539
  0xdac17f958d2ee523a2206206994597c13d831ec7
  []
  0x1
  0xc2ae3e86f07b6acbb36ab44c75f273d71abffd7e38d9fa2d6990f2708e338cfb
  0x123b773d3c056ae51819bee0d4533919907e163b28ff0e7538477d3dc9619ac2
]
```

Which can be added to an `authorization_list`:

```
[
  0x539
  0x
  0x163ef001
  0x81527974c
  0xf6f5
  0xaac09f958d2ee523a9906206994597c23d831ec8
  0x
  0xa9059cbb
  []
  [
    [
      0x539
      0xdac17f958d2ee523a2206206994597c13d831ec7
      []
      0x1
      0xc2ae3e86f07b6acbb36ab44c75f273d71abffd7e38d9fa2d6990f2708e338cfb
      0x123b773d3c056ae51819bee0d4533919907e163b28ff0e7538477d3dc9619ac2
    ]
  ]
]
```

And encoded and signed (as type `0x04`):

```shell
cat eip7702_tx_file | tx-util encode-tx --tx-type 4 | tx-util sign-tx --private-key 0x...
```

### Integer handling

Integers are expected to be hex-ecoded values (byte strings) following the Ethereum [spec](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/):

> For a positive integer, it is converted to the shortest byte array whose big-endian interpretation is the integer, and then encoded as a string according to the rules below.

### Booleans

Booleans are treated as integers as well, i.e., `true` is `0x01` while `false` is `0x80` (an empty byte string). 

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

**Also, this utility does not support error handling and should not be used e.g., in CI pipelines.**


## Getting help

Issues and PRs are welcome. We can also be reached at `gm@otim.xyz` if you have any additional questions.

