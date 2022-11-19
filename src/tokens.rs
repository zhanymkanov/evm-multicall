use std::collections::HashMap;

use crate::models::TargetToken;

fn get_trusted_tokens() -> [TargetToken; 6] {
    [
        TargetToken {
            chain_id: 1,
            address: String::from("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            symbol: Some(String::from("USDC")),
        },
        TargetToken {
            chain_id: 1,
            address: String::from("0xdAC17F958D2ee523a2206206994597C13D831ec7"),
            symbol: Some(String::from("USDT")),
        },
        TargetToken {
            chain_id: 1,
            address: String::from("0x6B175474E89094C44Da98b954EedeAC495271d0F"),
            symbol: Some(String::from("DAI")),
        },
        TargetToken {
            chain_id: 56,
            address: String::from("0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d"),
            symbol: Some(String::from("USDC")),
        },
        TargetToken {
            chain_id: 56,
            address: String::from("0x55d398326f99059fF775485246999027B3197955"),
            symbol: Some(String::from("USDT")),
        },
        TargetToken {
            chain_id: 56,
            address: String::from("0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56"),
            symbol: Some(String::from("BUSD")),
        },
    ]
}

pub fn get_chains_trusted_tokens() -> HashMap<i64, Vec<TargetToken>> {
    let tokens = get_trusted_tokens();

    let mut chain_trusted_tokens: HashMap<i64, Vec<TargetToken>> = HashMap::new();
    for token in tokens.into_iter() {
        chain_trusted_tokens
            .entry(token.chain_id)
            .or_insert_with(Vec::new)
            .push(token);
    }

    chain_trusted_tokens
}
