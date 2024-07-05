use alloy_primitives::{Bytes, U256, U64};
use core::panic;
use std::{collections::VecDeque, fmt};

#[derive(Clone)]
pub(crate) enum RlpItem {
    Data(Vec<u8>),
    List(Vec<RlpItem>),
}

#[allow(dead_code)]
impl RlpItem {
    pub(crate) fn data(&self) -> &[u8] {
        match self {
            RlpItem::Data(data) => data,
            _ => panic!("not data"),
        }
    }

    pub(crate) fn list(&self) -> &[RlpItem] {
        match self {
            RlpItem::List(list) => list,
            _ => panic!("not a list"),
        }
    }
}

impl From<&[u8]> for RlpItem {
    fn from(value: &[u8]) -> Self {
        RlpItem::Data(value.iter().cloned().collect::<Vec<_>>())
    }
}

impl From<bool> for RlpItem {
    fn from(value: bool) -> Self {
        RlpItem::Data(if value { vec![0x1] } else { vec![] })
    }
}

impl From<RlpItem> for bool {
    fn from(value: RlpItem) -> Self {
        match value.data() {
            [0x1] => true,
            [] => false,
            _ => panic!("invalid boolean value"),
        }
    }
}

impl From<U64> for RlpItem {
    fn from(value: U64) -> Self {
        RlpItem::Data(
            value
                .to_be_bytes::<8>()
                .into_iter()
                .skip_while(|b| *b == 0x0)
                .collect::<Vec<_>>(),
        )
    }
}

impl From<RlpItem> for U64 {
    fn from(value: RlpItem) -> Self {
        U64::from_be_slice(value.data())
    }
}

impl From<U256> for RlpItem {
    fn from(value: U256) -> Self {
        RlpItem::Data(
            value
                .to_be_bytes::<32>()
                .into_iter()
                .skip_while(|b| *b == 0x0)
                .collect::<Vec<_>>(),
        )
    }
}

impl From<RlpItem> for U256 {
    fn from(value: RlpItem) -> Self {
        U256::from_be_slice(value.data())
    }
}

impl From<Bytes> for RlpItem {
    fn from(value: Bytes) -> Self {
        RlpItem::Data(value.iter().cloned().collect::<Vec<u8>>().as_slice().into())
    }
}

impl From<RlpItem> for Vec<u8> {
    fn from(value: RlpItem) -> Self {
        let mut bytes = Vec::new();
        match value {
            RlpItem::Data(mut data) => match data.len() {
                1 if data[0] <= 0x7F => {
                    bytes.push(data[0]);
                }
                0..=55 => {
                    bytes.push(0x80 + data.len() as u8);
                    bytes.append(&mut data);
                }
                56.. => {
                    let mut len = data
                        .len()
                        .to_be_bytes()
                        .into_iter()
                        .skip_while(|b| *b == 0x0)
                        .collect::<Vec<_>>();
                    bytes.push(0xB7 + len.len() as u8);
                    bytes.append(&mut len);
                    bytes.append(&mut data);
                }
            },
            RlpItem::List(list) => {
                let mut encoded = list
                    .into_iter()
                    .flat_map(|item| Into::<Vec<u8>>::into(item))
                    .collect::<Vec<_>>();
                match encoded.len() {
                    0..=55 => {
                        bytes.push(0xC0 + encoded.len() as u8);
                        bytes.append(&mut encoded);
                    }
                    56.. => {
                        let mut len = encoded
                            .len()
                            .to_be_bytes()
                            .into_iter()
                            .skip_while(|b| *b == 0x0)
                            .collect::<Vec<_>>();
                        bytes.push(0xF7 + len.len() as u8);
                        bytes.append(&mut len);
                        bytes.append(&mut encoded);
                    }
                }
            }
        }
        bytes
    }
}

impl From<&mut VecDeque<u8>> for RlpItem {
    fn from(value: &mut VecDeque<u8>) -> Self {
        let byte = value.pop_front().expect("no more bytes");
        match byte {
            0x00..=0x7F => RlpItem::Data(vec![byte]),
            0x80..=0xBF => {
                let len = match byte {
                    0x80..=0xB7 => byte as u64 - 0x80,
                    0xB8..=0xBF => {
                        let len = byte - 0xB7;
                        let len = value.drain(0..len as usize).collect::<Vec<_>>();
                        len.into_iter().fold(0u64, |a, b| a * 256 + b as u64)
                    }
                    _ => unreachable!(),
                };
                let item = value.drain(0..len as usize).collect::<Vec<_>>();
                RlpItem::Data(item)
            }
            0xC0..=0xFF => {
                let len = match byte {
                    0xC0..=0xF7 => byte as u64 - 0xC0,
                    0xF8..=0xFF => {
                        let len = byte - 0xF7;
                        let len = value.drain(0..len as usize).collect::<Vec<_>>();
                        len.into_iter().fold(0u64, |a, b| a * 256 + b as u64)
                    }
                    _ => unreachable!(),
                };
                let mut items = value.drain(0..len as usize).collect::<VecDeque<_>>();
                let mut rlp_vals = Vec::new();
                while !items.is_empty() {
                    rlp_vals.push(Into::<RlpItem>::into(&mut items));
                }
                RlpItem::List(rlp_vals)
            }
        }
    }
}

impl fmt::Debug for RlpItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_rlp(item: &RlpItem, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
            match item {
                RlpItem::Data(data) => {
                    if data.is_empty() {
                        write!(f, "{:indent$}0x", "", indent = depth)
                    } else {
                        write!(
                            f,
                            "{:indent$}0x{}",
                            "",
                            hex::encode(data).trim_start_matches('0'),
                            indent = depth
                        )
                    }
                }
                RlpItem::List(list) => match list.len() {
                    0 => write!(f, "{:indent$}[]", "", indent = depth),
                    _ => {
                        writeln!(f, "{:indent$}[", "", indent = depth)?;
                        for item in list.iter() {
                            fmt_rlp(item, f, depth + 2)?;
                            writeln!(f)?;
                        }
                        write!(f, "{:indent$}]", "", indent = depth)
                    }
                },
            }
        }
        fmt_rlp(self, f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let a: RlpItem = true.into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: bool = a.into();
        assert_eq!(a, true);

        let a: RlpItem = false.into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: bool = a.into();
        assert_eq!(a, false);
    }

    #[test]
    fn test_u64() {
        let a: RlpItem = U64::from(0u64).into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: U64 = a.into();
        assert_eq!(a, U64::from(0u64));

        let a: RlpItem = U64::from(123456u64).into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: U64 = a.into();
        assert_eq!(a, U64::from(123456u64));
    }

    #[test]
    fn test_u256() {
        let a: RlpItem = U256::from(0u64).into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: U256 = a.into();
        assert_eq!(a, U256::from(0u64));

        let a: RlpItem = U256::from(123456u64).into();
        let a: Vec<u8> = a.into();
        let mut a = VecDeque::<u8>::from(a);
        let a = Into::<RlpItem>::into(&mut a);
        let a: U256 = a.into();
        assert_eq!(a, U256::from(123456u64));
    }
}
