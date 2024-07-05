use crate::rlp::RlpItem;
use alloy_primitives::{Address, Bytes, FixedBytes, U256, U64};
use k256::ecdsa::{signature::hazmat::PrehashSigner, SigningKey};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::vec;

const EIP1559_TX_TYPE: u8 = 2;
const EIP7702_TX_TYPE: u8 = 4;
const AUTHORIZATION_MAGIC: u8 = 5;

/// An [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) Transaction
/// ```no_run
/// 0x02 || rlp([
///   chain_id,
///   nonce,
///   max_priority_fee_per_gas,
///   max_fee_per_gas,
///   gas_limit,
///   destination,
///   amount,
///   data,
///   access_list,
///   y_parity,
///   r,
///   s
/// ])
/// ```
#[allow(missing_docs)]
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Eip1559 {
    pub(crate) chain_id: U256,
    pub(crate) nonce: U64,
    pub(crate) max_priority_fee_per_gas: U256,
    pub(crate) max_fee_per_gas: U256,
    pub(crate) gas_limit: U256,
    pub(crate) destination: Address,
    pub(crate) amount: U256,
    pub(crate) data: Bytes,
    pub(crate) access_list: Vec<AccessListItem>,
    #[serde(flatten)]
    pub(crate) signature: Option<Signature>,
}

/// An [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702) Transaction
///
/// See [`Authorization`] for `authorization_list`
/// ```no_run
/// 0x04 || rlp([
///   chain_id,
///   nonce,
///   max_priority_fee_per_gas,
///   max_fee_per_gas,
///   gas_limit,
///   destination,
///   amount,
///   data,
///   access_list,
///   authorization_list,
///   y_parity,
///   r,
///   s
/// ])
/// ```
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Eip7702 {
    pub(crate) chain_id: U256,
    pub(crate) nonce: U64,
    pub(crate) max_priority_fee_per_gas: U256,
    pub(crate) max_fee_per_gas: U256,
    pub(crate) gas_limit: U256,
    pub(crate) destination: Address,
    pub(crate) amount: U256,
    pub(crate) data: Bytes,
    pub(crate) access_list: Vec<AccessListItem>,
    pub(crate) authorization_list: Vec<Authorization>,
    #[serde(flatten)]
    pub(crate) signature: Option<Signature>,
}

/// An [EIP-2930](https://eips.ethereum.org/EIPS/eip-2930) Access List Item
/// ```no_run
/// rlp([
///   address,
///   [
///     storage_key,
///     ...
///   ]
/// ])
/// ```
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AccessListItem {
    pub(crate) address: Address,
    pub(crate) storage_keys: Vec<FixedBytes<32>>,
}

/// An [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702) Authorization
/// ```no_run
/// rlp([
///   chain_id,
///   address,
///   [
///     nonce
///   ],
///   y_parity,
///   r,
///   s
/// ])
/// ```
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Authorization {
    pub(crate) chain_id: U256,
    pub(crate) address: Address,
    pub(crate) nonce: Option<U64>,
    #[serde(flatten)]
    pub(crate) signature: Option<Signature>,
}

/// A Signature
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Signature {
    pub(crate) y_parity: bool,
    pub(crate) r: U256,
    pub(crate) s: U256,
}

impl From<Eip1559> for RlpItem {
    fn from(value: Eip1559) -> Self {
        let mut items = Vec::new();
        items.push(value.chain_id.into());
        items.push(value.nonce.into());
        items.push(value.max_priority_fee_per_gas.into());
        items.push(value.max_fee_per_gas.into());
        items.push(value.gas_limit.into());
        items.push(value.destination.as_slice().into());
        items.push(value.amount.into());
        items.push(value.data.into());
        items.push(value.access_list.into());
        if let Some(signature) = value.signature {
            let mut rlp: Vec<RlpItem> = signature.into();
            items.append(&mut rlp);
        }
        RlpItem::List(items)
    }
}

impl From<Eip7702> for RlpItem {
    fn from(value: Eip7702) -> Self {
        let mut items = Vec::new();
        items.push(value.chain_id.into());
        items.push(value.nonce.into());
        items.push(value.max_priority_fee_per_gas.into());
        items.push(value.max_fee_per_gas.into());
        items.push(value.gas_limit.into());
        items.push(value.destination.as_slice().into());
        items.push(value.amount.into());
        items.push(value.data.into());
        items.push(value.access_list.into());
        items.push(value.authorization_list.into());
        if let Some(signature) = value.signature {
            let mut rlp: Vec<RlpItem> = signature.into();
            items.append(&mut rlp);
        }
        RlpItem::List(items)
    }
}

impl From<Vec<AccessListItem>> for RlpItem {
    fn from(value: Vec<AccessListItem>) -> Self {
        let mut items = Vec::new();
        for item in value {
            items.push(RlpItem::List(vec![
                RlpItem::Data(item.address.as_slice().into()),
                RlpItem::List(
                    item.storage_keys
                        .into_iter()
                        .map(|k| RlpItem::Data(k.as_slice().into()))
                        .collect::<Vec<_>>(),
                ),
            ]))
        }
        RlpItem::List(items)
    }
}

impl From<Vec<Authorization>> for RlpItem {
    fn from(value: Vec<Authorization>) -> Self {
        let mut items = Vec::new();
        for item in value {
            items.push(item.into())
        }
        RlpItem::List(items)
    }
}

impl From<Authorization> for RlpItem {
    fn from(value: Authorization) -> Self {
        let mut items = Vec::new();
        items.push(value.chain_id.into());
        items.push(value.address.as_slice().into());
        // EIP-7702 optional nonce is encoded as an empty list
        items.push(RlpItem::List(
            value.nonce.map(|n| vec![n.into()]).unwrap_or(vec![]),
        ));
        if let Some(signature) = value.signature {
            let mut rlp: Vec<RlpItem> = signature.into();
            items.append(&mut rlp);
        }
        RlpItem::List(items)
    }
}

impl From<Signature> for Vec<RlpItem> {
    fn from(value: Signature) -> Self {
        let mut items = Vec::new();
        items.push(value.y_parity.into());
        items.push(value.r.into());
        items.push(value.s.into());
        items
    }
}

fn sign_payload(mut payload: Vec<u8>, magic: u8, signer: Vec<u8>) -> Signature {
    payload.insert(0, magic);

    let mut hasher = Keccak256::new();
    hasher.update(&payload);
    let hash = hasher.finalize();

    let signer = SigningKey::from_slice(&signer).unwrap();
    let (signature, recovery_id) = signer.sign_prehash(&hash).unwrap();

    Signature {
        y_parity: recovery_id.is_y_odd(),
        r: U256::from_be_slice(signature.r().to_bytes().as_slice()),
        s: U256::from_be_slice(signature.s().to_bytes().as_slice()),
    }
}

impl Authorization {
    pub(crate) fn sign(self, signer: Vec<u8>) -> Self {
        let mut auth = self.clone();
        auth.signature = None;

        let rlp: RlpItem = auth.clone().into();

        auth.signature = Some(sign_payload(rlp.into(), AUTHORIZATION_MAGIC, signer));
        auth
    }
}

impl Eip1559 {
    pub(crate) fn sign(self, signer: Vec<u8>) -> Self {
        let mut tx = self.clone();
        tx.signature = None;

        let rlp: RlpItem = tx.clone().into();

        tx.signature = Some(sign_payload(rlp.into(), EIP1559_TX_TYPE, signer));
        tx
    }
}

impl Eip7702 {
    pub(crate) fn sign(self, signer: Vec<u8>) -> Self {
        let mut tx = self.clone();
        tx.signature = None;

        let rlp: RlpItem = tx.clone().into();

        tx.signature = Some(sign_payload(rlp.into(), EIP7702_TX_TYPE, signer));
        tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EIP_1559_UNSIGNED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip1559_unsigned.json"
    ));

    static EIP_1559_SIGNED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip1559_signed.json"
    ));

    static EIP_1559_HEX_VALS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip1559_hex_vals.json"
    ));

    static EIP_7702_UNSIGNED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip7702_unsigned.json"
    ));

    static EIP_7702_SIGNED: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip7702_signed.json"
    ));

    static EIP_7702_EMPTY_AUTH: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/transactions/eip7702_empty_auth.json"
    ));

    #[test]
    fn deserialize_eip1559() {
        // valid tx
        let _tx: Eip1559 = serde_json::from_str(EIP_1559_UNSIGNED).unwrap();

        // signed tx
        let _tx: Eip1559 = serde_json::from_str(EIP_1559_SIGNED).unwrap();
    }

    #[test]
    fn deserialize_eip1559_hex() {
        let _tx: Eip1559 = serde_json::from_str(EIP_1559_HEX_VALS).unwrap();
    }

    #[test]
    fn deserialize_eip7702() {
        let _tx: Eip7702 = serde_json::from_str(EIP_7702_UNSIGNED).unwrap();

        // signed
        let _tx: Eip7702 = serde_json::from_str(EIP_7702_SIGNED).unwrap();

        // empty auth
        let _tx: Eip7702 = serde_json::from_str(EIP_7702_EMPTY_AUTH).unwrap();
    }
}
