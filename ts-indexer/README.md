# Overview

This is a custom indexer implemented in TypeScript and using sqlite. Compare with the [Rust indexer](../rust-indexer/README.md), this indexer is simpler and easier to write, but less performant, so we recommend using this for rapid prototyping and testing, and then move to the Rust version for production.

> [!WARNING]
> We use a hardcoded version of `aptos-proto` because only that version has the most up to date proto definition.
> We will release an official version of `aptos-proto` soon, for now just don't change the `aptos-proto` version in `package.json`.

## Usage

Make a copy of `.env.example` and rename it to `.env`, then fill in the values.

Run the following command to start the indexer:

```sh
npm i && npm run indexing
```

## Acknowledgements

Ported over from https://github.com/lithdew/aptos-indexer
