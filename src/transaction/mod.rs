use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Transaction {
    Eip1559(Eip1559),
    Eip7702(Eip7702),
}

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
#[derive(Default, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eip1559 {
    #[validate(range(min = 2, max = 2))]
    #[serde(rename = "type")]
    pub tx_type: u8,
    pub chain_id: U256,
    pub nonce: u64,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: U256,
    pub destination: Address,
    pub amount: U256,
    pub data: Bytes,
    pub access_list: Vec<AccessListItem>,
    #[serde(flatten)]
    pub signature: Option<Signature>,
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
#[derive(Default, Validate, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Eip7702 {
    #[validate(range(min = 4, max = 4))]
    #[serde(rename = "type")]
    pub tx_type: u8,
    pub chain_id: U256,
    pub nonce: u64,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: U256,
    pub destination: Address,
    pub amount: U256,
    pub data: Bytes,
    pub access_list: Vec<AccessListItem>,
    pub authorization_list: Vec<Authorization>,
    #[serde(flatten)]
    pub signature: Option<Signature>,
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
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListItem {
    pub address: Address,
    pub storage_keys: Vec<FixedBytes<32>>,
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
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorization {
    pub chain_id: U256,
    pub address: Address,
    pub nonce: Option<u64>,
    #[serde(flatten)]
    pub signature: Option<Signature>,
}

/// A Signature
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    pub y_parity: bool,
    pub r: U256,
    pub s: U256,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TX_EIP1559: &str = r#"
        {
            "type": 2,
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
            ]
        }
    "#;

    const TX_EIP1559_SIGNED: &str = r#"
        {
            "type": 2,
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
            "yParity": true,
            "r": "0x52ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87c",
            "s": "0x5a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5"
        }
    "#;

    const TX_EIP7702: &str = r#"
        {
            "type": 4,
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
                    "nonce": null
                }
            ]
        }
    "#;

    const TX_EIP7702_SIGNED: &str = r#"
        {
            "type": 4,
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
                    "nonce": null,
                    "yParity": true,
                    "r": "0x52ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87c",
                    "s": "0x5a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5"
                }
            ],
            "yParity": true,
            "r": "0x52ee022a326abb33e6bebab1fa694043371ab41a7a985ea23d48bd78502be87c",
            "s": "0x5a0f69dc8009a1e449bfbc8b13220bc40337da1325c261afdac1803f26d0e9d5"
        }
    "#;

    #[test]
    fn deserialize_eip1559_tx() {
        // valid tx
        let mut tx: Eip1559 = serde_json::from_str(TX_EIP1559).unwrap();
        assert!(tx.validate().is_ok());

        // wrong type
        tx.tx_type = 3;
        assert!(tx.validate().is_err());

        // signed tx
        let tx: Eip1559 = serde_json::from_str(TX_EIP1559_SIGNED).unwrap();
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn deserialize_eip7702_tx() {
        let tx: Eip7702 = serde_json::from_str(TX_EIP7702).unwrap();
        assert!(tx.validate().is_ok());

        // signed
        let tx: Eip7702 = serde_json::from_str(TX_EIP7702_SIGNED).unwrap();
        assert!(tx.validate().is_ok());
    }

    #[test]
    fn deserialize_any_tx() {
        // valid 1559 tx
        let _tx: Transaction = serde_json::from_str(TX_EIP1559).unwrap();

        // valid 7702 tx
        let _tx: Transaction = serde_json::from_str(TX_EIP7702).unwrap();
    }
}
