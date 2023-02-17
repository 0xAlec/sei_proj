Built with [Sylvia](https://github.com/CosmWasm/sylvia) framework

## How to Build

`$ rustup target add wasm32-unknown-unknown`

`$ cd src`

`$ cargo build --release --target wasm32-unknown-unknown --lib`

## Test Build

`$ cosmwasm-check target/wasm32-unknown-unknown/release/sei_proj.wasm`

## Unit/Integration Tests

`$ cargo test`

## Contract functions

- ```instantiate``` sets the deployer/sender as the contract admin and a specified denomination as the allowed coin denomination for the contract.
- ```get_owner``` retrieves the owner of the smart contract.
- ```transfer``` allows a user to split coins it sends to the contract to 2 recipients, with the coins stored for later withdrawal. The contract keeps track of sent coins through an internal balance mapping. Since integer division is used to implement splitting, remainders of odd sends are incurred as "send fees" to the contract admin, and are withdrawable by the owner.
- ```withdraw``` allows a user to withdraw up to their maximum allocated balance of coins. It performs a check for sufficient user balance before decrementing the internal balance mapping and sending the tokens.
- ```get_balance``` retrieves the balance for a specified address from the internal mapping. (# of tokens escrowed, can be 0)