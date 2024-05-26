```text
Usage: tx-util [COMMAND]

Commands:
  encode-rlp       Transform values into RLP-encoded hex.
                       Accepts single hex values and (nested) whitespace-separated hex
                       values surrounded by `[` and `]`. Reads values from stdin.
                       Run `help encode-rlp` for examples
  decode-rlp       Decodes RLP-encoded hex values into a human-readable format
  encode-tx        Encodes an EIP-2718 transaction into a hex value.
                       Reads the same input format as `ecode-rlp` from stdin and
                       requires a transaction type
  decode-tx        Decode the hex value of an EIP-2718 rlp-encoded transaction from stdin
  sign-auth        Sign an unsigned EIP-7702 authorization
  sign-tx          Sign an unsigned EIP-2718 transaction supplied to stdin (e.g., as produced by `encode-tx`)
  recover-address  Recover address from a signed, RLP encoded transaction from stdin
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
