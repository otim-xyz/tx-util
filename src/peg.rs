use crate::rlp::RlpItem;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"

RawTx = { SOI ~ ( Data | List ) ~ EOI }

List = ${ 
    "[" ~ WHITESPACE * ~ "]" | 
    "[" ~ 
        WHITESPACE *  ~ 
        ( Data | List )  ~ 
        ( WHITESPACE +  ~ ( Data | List ) ) *  ~ 
        WHITESPACE *  ~ 
    "]" 
}

Data = @{ "0x" ~ ASCII_HEX_DIGIT * }

WHITESPACE = _{ " " | "\t" | NEWLINE }

"#]
pub(crate) struct RawTxParser;

fn rlp_ast(pair: Pair<'_, Rule>) -> RlpItem {
    match pair.as_rule() {
        Rule::List => {
            let mut items = Vec::new();
            for pair in pair.into_inner() {
                items.push(rlp_ast(pair));
            }
            RlpItem::List(items)
        }
        Rule::Data => {
            let val = pair.as_str().trim_start_matches("0x");
            let val = match val.len() {
                0 => vec![],
                _ if val.len() % 2 != 0 => hex::decode(&format!("0{}", val)).unwrap(),
                _ => hex::decode(val).unwrap(),
            };
            RlpItem::Data(val)
        }
        _ => unreachable!(),
    }
}

impl TryFrom<&str> for RlpItem {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pair = RawTxParser::parse(Rule::RawTx, value)
            .expect("parser failure")
            .next()
            .unwrap() // RawTx
            .into_inner()
            .next()
            .unwrap(); // inner Data | List

        Ok(rlp_ast(pair))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        [
            0x1
            0x
            0x163ef001
            0x81527974c
            0xf6f5
            0xdac17f958d2ee523a2206206994597c13d831ec7
            0x
            0xa9059cbb0000000000000000000000005a96834046c1dff63119eb0eed6330fc5007a1d700000000000000000000000000000000000000000000000000000001a1432720
            []
            0x
            0xc712423aef12a4175c67278fce3053db992fc0cf953474c15bf09c58bcdbd287
            0x14a542ec19c83c7e15804a6f5e1a5a987b22aaee3b813970dad60accfdd491b9
        ]
    "#;

    #[test]
    fn it_works() {
        let rlp: RlpItem = INPUT.try_into().unwrap();

        println!("{:?}", rlp);
    }
}
