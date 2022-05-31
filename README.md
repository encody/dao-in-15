# DAO in 15

Dead simple DAO-like smart contract that can be explained (and maybe even written) in 15 minutes.

Implements trivial membership, proposals, and voting.

# Required Software

- Rust 1.61 + cargo
- Node.js
- NEAR CLI 3.2

# Usage

## Scripts

### `build.sh`

Compiles the smart contract to a WebAssembly binary. The binary path is `./target/wasm32-unknown-unknown/release/near_smart_contract_rust_template.wasm`.

### `contract.sh <command> <...arguments>`

Calls the NEAR CLI, where `<dev-account>` is the account ID of the most recent dev deployment on testnet:

```txt
near <command> <dev-account> <...arguments>
```

### `deploy.sh <account-id>`

Deploys the most recently built WASM binary to `<account-id>` on mainnet, and calls the `new` function with arguments generated by `init-args.js`.

### `dev-deploy.sh [--force]`

Deploys the most recently built WASM binary to the dev account in `neardev/`, or to a new dev account if `neardev/` is not found or `--force` is set. Calls the `new` function with arguments generated by `init-args.js`.

# Authors

- Jacob Lindahl <jacob@near.foundation> [@sudo_build](https://twitter.com/sudo_build)
