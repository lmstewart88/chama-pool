# Chama

Collective diaspora investment, built on trust — now built on-chain.

Chama takes the pooled-contribution model diaspora communities already trust (chamas in Kenya, susu in Ghana, tontines across West Africa) and rebuilds it as a transparent smart contract on Stellar. A group backs a specific, real project — a borehole, a production run, a piece of equipment — contributions are tracked openly on-chain, and funds release automatically once the goal is met. No single gatekeeper holds the pot.

Built at London Builder HQ, September 2026.

## How it works

1. A project is posted with a funding goal and a recipient address
2. Diaspora contributors send funds to the pool via the Soroban smart contract
3. The contract tracks total contributions in real time
4. Once the goal is met, the full pool releases automatically to the recipient — no manual approval step

## What's here

- `contracts/pool` — the Soroban smart contract (`initialize`, `contribute`, `get_balance`, `get_goal`, `is_complete`), with tests covering below-goal contributions, goal-crossing auto-release, and double-initialization protection
- `index.html` — a single-page demo front end that connects to the deployed contract via [Freighter](https://www.freighter.app/) and shows live pool progress

## Live demo (Stellar testnet)

- Contract: [`CBJNEVBLXC7LYXHXUWONPS2BRSZUJED23AEYM2NB66GT35XPKVD5GZZC`](https://stellar.expert/explorer/testnet/contract/CBJNEVBLXC7LYXHXUWONPS2BRSZUJED23AEYM2NB66GT35XPKVD5GZZC)
- Recipient: [`GDOQTV7O34ZRCUFQNSBZSKGGSCBM4KBD24DG4GNQOSMYLWQBAH64HFEN`](https://stellar.expert/explorer/testnet/account/GDOQTV7O34ZRCUFQNSBZSKGGSCBM4KBD24DG4GNQOSMYLWQBAH64HFEN)

To try it: open `index.html` in a browser with the [Freighter wallet extension](https://www.freighter.app/) installed, set it to testnet, connect, and contribute XLM. Progress updates live; crossing the goal releases the full pool to the recipient automatically.

## Why Stellar

Fast settlement, near-zero fees, and Soroban smart contracts mean the rules of a chama — contribute, track, release — can be enforced by code instead of by trust in one person holding the money. Recipients don't need a Stellar wallet to eventually receive local currency: Stellar's anchor network (built on open SEP standards) connects on-chain funds to local payment rails like M-Pesa in Kenya, so cash-out to the ground is a standard integration, not a hypothetical.

## Roadmap

- Connect a Stellar anchor for M-Pesa cash-out to recipients
- Support multiple simultaneous project pools
- Refund path for pools that don't meet their goal by a deadline
- Recipient identity verification for real-world deployment

## Local development

```bash
# build and test the contract
cd contracts/pool
cargo test

# from the workspace root
stellar contract build
stellar contract deploy \
  --wasm target/wasm32v1-none/release/pool.wasm \
  --source-account <your-identity> \
  --network testnet \
  --alias pool
```

Update `CONTRACT_ID` and `RECIPIENT_ID` at the top of `index.html`'s script block to point at your own deployment.
