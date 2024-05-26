use core::panic;
use std::{collections::VecDeque, fmt};

#[derive(Clone)]
pub(crate) enum RlpItem {
    Data(Vec<u8>),
    List(Vec<RlpItem>),
}

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

pub(crate) fn encode_rlp(item: RlpItem) -> Vec<u8> {
    let mut bytes = Vec::new();
    match item {
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
                .map(|item| encode_rlp(item))
                .flatten()
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

pub(crate) fn decode_rlp(bytes: &mut VecDeque<u8>) -> RlpItem {
    let byte = bytes.pop_front().expect("no more bytes");
    match byte {
        0x00..=0x7F => RlpItem::Data(vec![byte]),
        0x80..=0xBF => {
            let len = match byte {
                0x80..=0xB7 => byte as u64 - 0x80,
                0xB8..=0xBF => {
                    let len = byte - 0xB7;
                    let len = bytes.drain(0..len as usize).collect::<Vec<_>>();
                    len.into_iter().fold(0u64, |a, b| a * 256 + b as u64)
                }
                _ => unreachable!(),
            };
            let item = bytes.drain(0..len as usize).collect::<Vec<_>>();
            RlpItem::Data(item)
        }
        0xC0..=0xFF => {
            let len = match byte {
                0xC0..=0xF7 => byte as u64 - 0xC0,
                0xF8..=0xFF => {
                    let len = byte - 0xF7;
                    let len = bytes.drain(0..len as usize).collect::<Vec<_>>();
                    len.into_iter().fold(0u64, |a, b| a * 256 + b as u64)
                }
                _ => unreachable!(),
            };
            let mut items = bytes.drain(0..len as usize).collect::<VecDeque<_>>();
            let mut rlp_vals = Vec::new();
            while !items.is_empty() {
                rlp_vals.push(decode_rlp(&mut items));
            }
            RlpItem::List(rlp_vals)
        }
    }
}

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
                write!(f, "{:indent$}[\n", "", indent = depth)?;
                for item in list.iter() {
                    fmt_rlp(&item, f, depth + 2)?;
                    write!(f, "\n")?;
                }
                write!(f, "{:indent$}]", "", indent = depth)
            }
        },
    }
}

impl fmt::Debug for RlpItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_rlp(self, f, 0)
    }
}
