mod chains;
mod models;
mod tokens;

use std::collections::HashMap;

use futures::stream::FuturesUnordered;
use futures::{FutureExt, StreamExt};

use models::TargetToken;
use tokens::get_chains_tokens;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chain_trusted_tokens: HashMap<i64, Vec<TargetToken>> = get_chains_tokens();
    let address = "0xB25C5E8fA1E53eEb9bE3421C59F6A66B786ED77A";

    let mut tasks = FuturesUnordered::new();
    for chain in &chains::CHAINS {
        let trusted_tokens = chain_trusted_tokens.get(&chain.id).unwrap();
        tasks.push(chain.get_balances(address, trusted_tokens).boxed());
    }
    while !tasks.is_empty() {
        tasks.next().await;
    }
    Ok(())
}
