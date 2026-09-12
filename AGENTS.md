# Agent instructions

This is a Stellar smart-contract workspace (Soroban). Each contract is a workspace member under `contracts/<name>/`.

## Layout
- `Cargo.toml` — workspace root; contract crates inherit `soroban-sdk` from here
- `contracts/<name>/src/lib.rs` — contract implementation (`#![no_std]`)
- `contracts/<name>/src/test.rs` — host-side unit tests

## Build

From the workspace root:

```sh
stellar contract build
```

That compiles every `cdylib` member to WASM. Artifacts land in `target/wasm32v1-none/release/*.wasm`. Build one crate with `stellar contract build --package <name>`.

Do not substitute this with `cargo build --target wasm32v1-none`. `stellar contract build` applies the flags and metadata the network expects.

The `wasm32v1-none` Rust target must be installed (`rustup target add wasm32v1-none`). Rust 1.84 or newer is required for that target. Rust 1.82 and 1.83 cannot build contracts.

## Test

Host tests run with the normal Cargo test harness (not on-chain):

```sh
cargo test
```

A single crate: `cargo test -p <name>`.

## Deploy and invoke

On testnet, after a successful build:

```sh
stellar contract deploy \
  --wasm target/wasm32v1-none/release/<name>.wasm \
  --source-account <identity> \
  --network testnet \
  --alias <alias>

stellar contract invoke \
  --id <alias> \
  --network testnet \
  --source-account <identity> \
  -- hello --to world
```

The sample `hello_world` contract exposes `hello(to: String) -> Vec<String>`. Replace that with your own functions; `stellar contract invoke --id <id> -- -h` prints the generated CLI for the deployed contract.

## Further reading

- https://developers.stellar.org/docs/build/smart-contracts/overview
- https://github.com/stellar/soroban-examples

## Contract Spec: Chama Pool

This workspace contains one contract, at `contracts/pool`, implementing a pooled-contribution fund (a "chama" — a diaspora collective investment pool).

### Purpose
Multiple contributors send funds toward a shared goal backing a real-world project (e.g. agricultural infrastructure). Once total contributions reach the goal, funds release automatically to a designated recipient — no manual approval step, no intermediary holding funds.

### Required functions

- `initialize(env: Env, admin: Address, recipient: Address, token: Address, goal: i128)`
  - Sets up the pool: who administers it, who receives funds on success, which token is used (e.g. native XLM or a SEP-41 token), and the funding target.
  - Should only be callable once — store an `initialized` flag and panic if called again.

- `contribute(env: Env, from: Address, amount: i128)`
  - Requires `from.require_auth()`.
  - Transfers `amount` of the pool's token from `from` to the contract itself, using the token client (not just internal bookkeeping — this must move real balances).
  - Adds `amount` to a running total in persistent storage.
  - If the new total >= goal AND the pool isn't already marked complete: transfer the full pooled balance from the contract to `recipient` using the token client, and mark the pool as complete.

- `get_balance(env: Env) -> i128`
  - Returns current total raised.

- `get_goal(env: Env) -> i128`
  - Returns the funding target set at initialization.

- `is_complete(env: Env) -> bool`
  - Returns whether the goal has been met and funds released.

### Storage
Use persistent storage for goal, total raised, recipient, token address, and completion status — this needs to survive between calls.

### Tests
Include tests in `test.rs` covering:
1. A contribution below the goal — total updates, pool stays incomplete, no transfer to recipient.
2. A contribution that meets or exceeds the goal — total updates, pool marked complete, recipient's balance increases by the pooled amount.
3. Calling `initialize` twice — should panic or return an error.

### Non-goals for this version
No refund logic, no multi-token support, no rotation/turn-taking mechanics. Keep it to contribute → track → release.




