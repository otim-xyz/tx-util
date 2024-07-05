//! # tx-util

#![warn(
    missing_docs,
    non_ascii_idents,
    unreachable_pub,
    unused_crate_dependencies,
    unused_results,
    unused_qualifications,
    nonstandard_style,
    rustdoc::all
)]
#![deny(rust_2018_idioms, unsafe_code)]

mod rlp;
mod transaction;

use crate::rlp::RlpItem;
use clap::{CommandFactory, Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};
use std::{io, iter::zip};
use transaction::{Eip1559, Eip7702};

#[cfg(test)]
use assert_cmd as _;

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
    /// Encodes an EIP-2718 transaction into an rlp-encoded hex value from stdin.
    ///
    /// Accepts json input with a `type` field followed by valid tranaction fields.
    ///
    /// This currently accepts types `2` and `4` only.
    ///
    /// ```no_run
    /// {
    ///     "type": 2,
    ///     "chainId": 1337,
    ///     "nonce": 0,
    ///     ...
    /// }
    /// ```
    #[command(long_about, verbatim_doc_comment)]
    EncodeTx {
        /// Transaction type. Types `2` and `4` accepted.
        #[arg(long, short = 't')]
        tx_type: u8,

        /// A private key in hex encoding `0x...`. This is required
        /// if the transaction does not contain a signature.
        #[arg(long)]
        signer: Option<String>,

        /// For type 4 transactions only.
        ///
        /// A list of private keys in hex encoding `0x...`. These are
        /// required if the elements of the `authorization_list` are not
        /// already signed.
        ///
        /// If present, the number of keys supplied here must be equal to
        /// the number of items in the `authorization_list`.
        #[arg(long = "authorizer")]
        authorizers: Vec<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Some(Commands::EncodeTx {
            tx_type,
            signer,
            authorizers,
        }) => match tx_type {
            0x2 => {
                let stdin = io::read_to_string(io::stdin())?;
                let tx: Eip1559 = serde_json::from_str(stdin.trim())?;
                let ast: RlpItem = if tx.signature.is_none() {
                    let signer =
                        signer.ok_or(eyre!("a `--signer` is required to sign this transaction"))?;
                    let signer = hex::decode(signer.trim().trim_start_matches("0x"))?;
                    if signer.len() != 32 {
                        Err(eyre!("the supplied `--signer` is invalid"))?;
                    }
                    tx.sign(signer).into()
                } else {
                    tx.into()
                };
                let mut bytes: Vec<u8> = ast.into();
                bytes.insert(0, 2);
                print!("0x{}", hex::encode(bytes));
            }
            0x4 => {
                let stdin = io::read_to_string(io::stdin())?;
                let mut tx: Eip7702 = serde_json::from_str(stdin.trim())?;
                if tx.authorization_list.iter().any(|a| a.signature.is_none()) {
                    if tx.authorization_list.len() != authorizers.len() {
                        Err(eyre!("the number of `--authorizer` must be equal to the number of items in the `authorization_list`"))?;
                    }
                    let mut signers = Vec::new();
                    for a in authorizers {
                        let signer = hex::decode(a.trim().trim_start_matches("0x"))?;
                        if signer.len() != 32 {
                            Err(eyre!("a supplied `--authorizer` is invalid"))?;
                        }
                        signers.push(signer);
                    }
                    tx.authorization_list = zip(tx.authorization_list, signers)
                        .map(|(auth, signer)| auth.sign(signer))
                        .collect::<Vec<_>>();
                }
                let ast: RlpItem = if tx.signature.is_none() {
                    let signer =
                        signer.ok_or(eyre!("a `--signer` is required to sign this transaction"))?;
                    let signer = hex::decode(signer.trim().trim_start_matches("0x"))?;
                    tx.sign(signer).into()
                } else {
                    tx.into()
                };
                let mut bytes: Vec<u8> = ast.into();
                bytes.insert(0, 4);
                print!("0x{}", hex::encode(bytes));
            }
            _ => Err(eyre!("invalid transaction type`"))?,
        },
        None => Args::command().print_help().unwrap(),
    }
    Ok(())
}
