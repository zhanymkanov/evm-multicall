use web3::contract::tokens::{Detokenize, Tokenizable};
use web3::contract::Error;
use web3::ethabi::{Bytes, Token};
use web3::types::{Address, U256};

pub struct TargetToken {
    pub address: String,
    pub chain_id: i64,
    pub symbol: Option<String>,
}

pub struct CallData {
    pub target: Address,
    pub call_data: Bytes,
}
impl Tokenizable for CallData {
    fn from_token(token: Token) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match token {
            Token::Tuple(x) => {
                if x.len() != 2 {
                    return Err(Error::Api(web3::error::Error::Decoder(
                        "Invalid tuple".to_string(),
                    )));
                }
                Ok(CallData {
                    target: Address::from_token(x[0].clone()).unwrap(),
                    call_data: Bytes::from_token(x[1].clone()).unwrap(),
                })
            }
            _ => Err(Error::Api(web3::error::Error::Decoder("Error".to_string()))),
        }
    }
    fn into_token(self) -> Token {
        Token::Tuple(vec![self.target.into_token(), self.call_data.into_token()])
    }
}
impl CallData {
    pub fn get_param_calldata(self) -> Token {
        self.into_token()
    }
}

#[derive(Default)]
pub struct MulticallResponse {
    pub return_data: Vec<U256>,
}

impl Detokenize for MulticallResponse {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        /* Response:
        [block_number, [Response, Response, Response]]
        */
        let return_data = tokens[1].clone().into_array().unwrap();

        let token_balances = return_data
            .iter()
            .map(|response| match response {
                Token::Bytes(x) => U256::from_big_endian(&x),
                _ => U256([0, 0, 0, 0]),
            })
            .collect();

        Ok(Self {
            return_data: token_balances,
        })
    }
}
