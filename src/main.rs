//! # tx-util

#![warn(
    missing_docs,
    non_ascii_idents,
    // unreachable_pub,
    unused_crate_dependencies,
    unused_results,
    unused_qualifications,
    nonstandard_style,
    rustdoc::all
)]
#![deny(rust_2018_idioms, unsafe_code)]

mod peg;
mod rlp;
mod transaction;

use crate::rlp::{decode_rlp, encode_rlp, RlpItem};
use clap::{CommandFactory, Parser, Subcommand};
use k256::{
    ecdsa::{signature::hazmat::PrehashSigner, RecoveryId, Signature, SigningKey, VerifyingKey},
    FieldBytes,
};
use sha3::{Digest, Keccak256};
use std::{collections::VecDeque, io, vec};

/// WARNING !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
/// Do not use this for generating transactions for the Ethereum mainnet. This tool is
/// meant for generating test transactions only. It provides no guarantees of security
/// or correctness nor has it been audited. Use at your own risk.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, verbatim_doc_comment)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Transform values into RLP-encoded hex.
    /// Accepts single hex values and (nested) whitespace-separated hex
    /// values surrounded by `[` and `]`. Reads values from stdin.
    /// Run `help encode-rlp` for examples
    ///
    /// Examples:
    /// ```
    /// 0x0123456789ABCDEF
    /// ```
    /// ```
    /// [
    ///   0x1
    ///   0x2
    ///   [
    ///     0x3
    ///     0x4
    ///   ]
    ///   0x
    ///   []
    /// ]
    /// ```
    #[command(long_about, verbatim_doc_comment)]
    EncodeRlp,
    /// Decodes RLP-encoded hex values into a human-readable format
    DecodeRlp,
    /// Encodes an EIP-2718 transaction into a hex value.
    /// Reads the same input format as `ecode-rlp` from stdin and
    /// requires a transaction type
    #[command(long_about, verbatim_doc_comment)]
    EncodeTx {
        /// Transaction type
        #[arg(long, short = 't')]
        tx_type: u8,
    },
    /// Decode the hex value of an EIP-2718 rlp-encoded transaction from stdin
    DecodeTx,
    /// Sign an unsigned EIP-7702 authorization
    SignAuth {
        /// A private key in hex encoding
        #[arg(long, short = 'k')]
        private_key: String,

        /// The magic byte prepended to the rlp encoded payload
        #[arg(long, short = 'm', default_value_t = 0x5)]
        magic: u8,
    },
    /// Sign an unsigned EIP-2718 transaction supplied to stdin (e.g., as produced by `encode-tx`)
    SignTx {
        /// A private key in hex encoding
        #[arg(long, short = 'k')]
        private_key: String,
    },
    /// Recover address from a signed, RLP encoded transaction from stdin
    RecoverAddress,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::EncodeRlp) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let ast: RlpItem = stdin.trim().try_into().unwrap();
            let bytes = encode_rlp(ast);
            println!("0x{}", hex::encode(bytes));
        }
        Some(Commands::DecodeRlp) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let bytes = hex::decode(stdin.trim().trim_start_matches("0x")).unwrap();
            let rlp = decode_rlp(&mut VecDeque::from(bytes));
            println!("{:?}", rlp);
        }
        Some(Commands::EncodeTx { tx_type }) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let ast: RlpItem = stdin.trim().try_into().unwrap();
            let mut bytes = encode_rlp(ast);
            bytes.insert(0, tx_type);
            println!("0x{}", hex::encode(bytes));
        }
        Some(Commands::DecodeTx) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let mut bytes =
                VecDeque::from(hex::decode(stdin.trim().trim_start_matches("0x")).unwrap());
            let tx_type = bytes.pop_front().unwrap();
            let ast = decode_rlp(&mut bytes);
            println!("Transaction Type: 0x{}", hex::encode(vec![tx_type]));
            println!("Transaction Payload:");
            println!("{:?}", ast)
        }
        Some(Commands::SignAuth { private_key, magic }) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let mut payload = hex::decode(stdin.trim().trim_start_matches("0x")).unwrap();

            payload.insert(0, magic);

            let mut hasher = Keccak256::new();
            hasher.update(&payload);
            let hash = hasher.finalize();

            _ = payload.remove(0);

            let pk = hex::decode(private_key.trim_start_matches("0x")).unwrap();
            assert!(pk.len() == 32);
            let signer = SigningKey::from_slice(&pk).unwrap();
            let (signature, recovery_id) = signer.sign_prehash(&hash).unwrap();

            let y: RlpItem = RlpItem::Data(if recovery_id.to_byte() == 0x0 {
                vec![]
            } else {
                vec![0x1]
            });
            let r = RlpItem::Data(signature.r().to_bytes().to_vec());
            let s = RlpItem::Data(signature.s().to_bytes().to_vec());

            let mut bytes = VecDeque::from(payload);
            let ast = decode_rlp(&mut bytes);

            let mut items = match ast {
                RlpItem::List(items) => items,
                _ => panic!("invalid auth format"),
            };
            items.append(&mut vec![y, r, s]);

            let bytes = encode_rlp(RlpItem::List(items));

            println!("0x{}", hex::encode(bytes));
        }
        Some(Commands::SignTx { private_key }) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let payload = hex::decode(stdin.trim().trim_start_matches("0x")).unwrap();

            let mut hasher = Keccak256::new();
            hasher.update(&payload);
            let hash = hasher.finalize();

            let pk = hex::decode(private_key.trim_start_matches("0x")).unwrap();
            assert!(pk.len() == 32);
            let signer = SigningKey::from_slice(&pk).unwrap();
            let (signature, recovery_id) = signer.sign_prehash(&hash).unwrap();

            let y: RlpItem = RlpItem::Data(if recovery_id.to_byte() == 0x0 {
                vec![]
            } else {
                vec![0x1]
            });
            let r = RlpItem::Data(signature.r().to_bytes().to_vec());
            let s = RlpItem::Data(signature.s().to_bytes().to_vec());

            let mut bytes = VecDeque::from(payload);
            let tx_type = bytes.pop_front().unwrap();
            let ast = decode_rlp(&mut bytes);

            let mut items = match ast {
                RlpItem::List(items) => items,
                _ => panic!("invalid tx format"),
            };
            items.append(&mut vec![y, r, s]);

            let mut bytes = encode_rlp(RlpItem::List(items));
            bytes.insert(0, tx_type);

            println!("0x{}", hex::encode(bytes));
        }
        Some(Commands::RecoverAddress) => {
            let stdin = io::read_to_string(io::stdin()).unwrap();
            let mut bytes =
                VecDeque::from(hex::decode(stdin.trim().trim_start_matches("0x")).unwrap());

            let tx_type = bytes.pop_front().unwrap();
            let ast = decode_rlp(&mut bytes);

            let (payload, signature) = ast.list().split_at(ast.list().len() - 3);
            let y = if signature[0].data().is_empty() { 0 } else { 1 };
            let r = signature[1].data();
            let s = signature[2].data();

            let mut payload = encode_rlp(RlpItem::List(payload.into()));
            payload.insert(0, tx_type);

            let mut hasher = Keccak256::new();
            hasher.update(payload);
            let hash = hasher.finalize();

            let signature =
                Signature::from_scalars(*FieldBytes::from_slice(r), *FieldBytes::from_slice(s))
                    .unwrap();
            let recid = RecoveryId::try_from(y).unwrap();

            let recovered_key =
                VerifyingKey::recover_from_prehash(&hash, &signature, recid).unwrap();
            let encoded_point = recovered_key.to_encoded_point(false);

            let mut hasher = Keccak256::new();
            hasher.update(&encoded_point.as_bytes()[1..]);
            let address = &hasher.finalize()[12..];

            println!("Transaction Hash: 0x{}", hex::encode(hash));
            println!("Address: 0x{}", hex::encode(address));
        }
        None => Args::command().print_help().unwrap(),
    }
}
