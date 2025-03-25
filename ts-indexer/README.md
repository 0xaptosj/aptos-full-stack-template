# aptos-indexer

A barebones transaction streaming library for Aptos written in TypeScript. Designed for building indexers, analytics tools, and real-time transaction monitoring systems.

This was written as a result of the warning specified in the README of [the existing transaction streaming TypeScript SDK](https://github.com/aptos-labs/aptos-indexer-processors/tree/main/typescript) written by the Aptos team.

Profiling the existing transaction streaming TypeScript SDK revealed that the bottleneck which the Aptos team warns about isn't in gRPC's JavaScript client library or whichever JavaScript runtime they may have used, but rather the Aptos teams' choice of database and ORM library (PostgreSQL + TypeORM).

This library has been thoroughly tested by processing the complete transaction history of both Aptos Testnet and Mainnet using Bun and SQLite (via. `bun:sqlite`).

This library was built and tested using Bun v1.2.2.

## Prerequisites

- [Bun](https://bun.sh) v1.2.2 or later
- An Aptos API key from [Aptos Labs](https://aptoslabs.com/developers)

## Installation

```bash
bun install https://github.com/lithdew/aptos-indexer
```

## Quick Start

```typescript
import { streamTransactions } from "aptos-indexer";

// Stream transactions from version 0
for await (const event of streamTransactions({
  url: process.env.INDEXER_GRPC_URL!,
  apiKey: process.env.APTOS_API_KEY!,
  startingVersion: 0n,
})) {
  switch (event.type) {
    case "data": {
      console.debug(`Got ${event.transactions.length} transaction(s)`);
      break;
    }
    case "error": {
      console.error(event.error);
      break;
    }
  }
}
```

Check out the `examples` directory for complete working examples:

- [00_basic.ts](./examples/00_basic.ts): Basic transaction streaming
- [01_sqlite.ts](./examples/01_sqlite.ts): Transaction streaming with SQLite persistence
