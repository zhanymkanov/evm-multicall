use const_format::formatcp;
use std::str::FromStr;
use web3::api::Eth;
use web3::Transport;

use web3::contract::{Contract, Options};
use web3::ethabi::Token;
use web3::types::{Address, U256};

use crate::models::{CallData, MulticallResponse, TargetToken};

pub struct Chain {
    pub id: i64,
    pub multicall_address: &'static str,
    pub node_url: &'static str,
}
const ANKR_KEY: &str = "";

static MULTICALL_ABI: &[u8] = include_bytes!("../abi/multicall.json");
static ERC20_ABI: &[u8] = include_bytes!("../abi/erc20.json").as_slice();

impl Chain {
    pub async fn get_balances(&self, address: &str, trusted_tokens: &Vec<TargetToken>) {
        println!("Starting to get balances {}", self.id);

        let transport = web3::transports::Http::new(self.node_url).unwrap();
        let web3 = web3::Web3::new(transport);

        let caller_address = Address::from_str(address).unwrap();
        let tokens_addresses = self.parse_tokens(trusted_tokens);
        let tokens_balance: Vec<U256> = self
            .multicall(web3.eth(), caller_address, &tokens_addresses)
            .await;

        for (idx, balance) in tokens_balance.iter().enumerate() {
            println!(
                "Token {} of chain {} has balanceOf {:?}",
                &tokens_addresses[idx], self.id, balance
            );
        }
        println!("Finished to get balances {}", self.id);
    }

    fn parse_tokens(&self, trusted_tokens: &Vec<TargetToken>) -> Vec<Address> {
        trusted_tokens
            .iter()
            .filter_map(|trusted_token| {
                Address::from_str(trusted_token.address.as_str())
                    .map_err(|op| {
                        println!("Invalid address: {op}");
                        op
                    })
                    .ok()
            })
            .collect()
    }

    async fn multicall<T: Transport>(
        &self,
        eth: Eth<T>,
        caller_address: Address,
        tokens_addresses: &Vec<Address>,
    ) -> Vec<U256> {
        let contract_address = Address::from_str(self.multicall_address).unwrap();
        let contract_abi = MULTICALL_ABI;

        let token_contract = Contract::from_json(eth, contract_address, contract_abi).unwrap();

        let call_data = self.get_multicall_calldata(caller_address, tokens_addresses);
        let tokens_balance: MulticallResponse = token_contract
            .query("aggregate", call_data, None, Options::default(), None)
            .await
            .unwrap_or_else(|op| {
                println!("Failed to execute call to {} of chain {}", self.multicall_address, self.id);
                println!("Error: {op}");
                MulticallResponse::default()
            });

        tokens_balance.return_data
    }

    fn get_multicall_calldata(
        &self,
        caller_address: Address,
        tokens_addresses: &Vec<Address>,
    ) -> Vec<Token> {
        let erc20_contract = web3::ethabi::Contract::load(ERC20_ABI).unwrap();
        let function = erc20_contract.function("balanceOf").unwrap();
        let erc20_call_data = function
            .encode_input(&[Token::Address(caller_address)])
            .unwrap();

        tokens_addresses
            .iter()
            .map(|x| {
                CallData {
                    target: x.clone(),
                    call_data: erc20_call_data.clone(),
                }
                .get_param_calldata()
            })
            .collect::<Vec<Token>>()
    }
}

static ETHEREUM: Chain = Chain {
    id: 1,
    multicall_address: "0x5ba1e12693dc8f9c48aad8770482f4739beed696",
    node_url: formatcp!(
        "https://rpc.ankr.com/{chain}/{ankr_key}",
        chain = "eth",
        ankr_key = ANKR_KEY,
    ),
};

static BSC: Chain = Chain {
    id: 56,
    multicall_address: "0x15dc8b5ed578AA7a019dd0139B330cfD625cA795",
    node_url: formatcp!(
        "https://rpc.ankr.com/{chain}/{ankr_key}",
        chain = "bsc",
        ankr_key = ANKR_KEY,
    ),
};

pub static CHAINS: [&Chain; 2] = [&ETHEREUM, &BSC];
