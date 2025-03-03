# A template to build a full stack app on Aptos

This template is an opinionated alternative template to [CAD (create-aptos-dapp)](https://aptos.dev/en/build/create-aptos-dapp).

Please read each directory's README carefully to understand how to use the template.

- `indexer` directory for custom indexer on the contract
- `next-app` directory for the Next.js frontend
- `move` directory for the contract and integration tests
- `node-scripts` directory for some quick scripts to interact with the contract in Node.js

If you are on a Windows machine or have trouble pulling a docker image or connecting to RPC, you can use GitHub codespace, this repo is pre-configured for codespace with all the dependencies (Rust, docker, google cloud cli) ready. If you use codespace, you can install aptos-cli via 

```sh
curl -fsSL "https://aptos.dev/scripts/install_cli.py" | python3
```
