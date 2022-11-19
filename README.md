## Rust Web3 Multicall Example

Request address's balance for USDC, USDT, and DAI/BUSD on Ethereum and BSC with 2 RPC calls, instead of 6.

## How does it work
The program aggregates calls to each ERC-20's smart contract `balanceOf` methods and sends the bulk calldata to the chain's multicall contract.

Asynchronously, for each chain:
1. Init a list of tokens to parse (e.g. USDC, USDT, BUSD) from ETH and BSC.
2. Prepare ERC-20 calldata for each token smart contract.
3. Convert ERC-20 calldata to Multicall calldata.
3. Send calldata to multicall contract.
