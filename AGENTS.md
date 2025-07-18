# Repository Context

This document provides a concise overview of the repository structure and primary components for quick reference.

Refer to the main [README.md](README.md) for tutorial videos, detailed setup, and higher-level guidance.

## Top-Level Overview

This template helps you build a full-stack dApp on the Aptos blockchain. It includes:

- Move smart contracts and integration tests
- A Next.js frontend (React + Tailwind)
- Utility scripts in Node.js
- Custom blockchain indexers in TypeScript and Rust

## Directory Structure

| Directory      | Tech Stack               | Purpose                                                       |
| -------------- | ------------------------ | ------------------------------------------------------------- |
| `move`         | Move                     | Smart contracts (modules/packages) and integration tests      |
| `next-app`     | TypeScript, Next.js      | Frontend application with React, Next.js, and Tailwind CSS    |
| `node-script`  | TypeScript, Node.js      | Standalone scripts for interacting with contracts and DB      |
| `ts-indexer`   | TypeScript, SQLite       | Simple indexer for rapid prototyping (writes to local SQLite) |
| `rust-indexer` | Rust, PostgreSQL, Docker | Production-grade indexer for performance (writes to Postgres) |

## Root Files

- `README.md` : High-level overview, tutorial links, and usage guide
- `LICENSE` : Project license
- `CLAUDE.md` : Internal/meta notes

## Quick Reference Commands

See each subdirectory's README for detailed instructions. Common entry points:

```sh
# Move contracts: run integration tests
cd move && npm install && npx aptos-workspace test

# Frontend: start Next.js development server
cd next-app && npm install && npm run dev

# Node scripts: run available utility scripts
cd node-script && npm install && npm run <script>

# TypeScript indexer: start indexing with SQLite
cd ts-indexer && npm install && npm run indexing

# Rust indexer: run with Postgres (requires Docker / local Postgres)
cd rust-indexer && cargo run
```

## Further Reading

- Subdirectory READMEs contain configuration details, environment variables, and advanced usage.
- Main README links to video tutorials (English/中文) and codespace setup.
