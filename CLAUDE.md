# Aptos Full Stack Template - AI Assistant Context

## Project Overview

This is a full-stack Aptos dApp template featuring a message board application with custom indexers. It serves as an alternative to the official CAD (create-aptos-dapp) template with opinionated architecture choices.

## Repository Structure

### Move Contract (`/move/`)

- **Location**: `move/contracts/message-board/`
- **Main contract**: `sources/message_board.move`
- **Test command**: `cd move/contracts/message-board && npm test`
- **Deploy command**: `cd move/contracts/message-board && ./sh_scripts/deploy.sh`
- **Format command**: `cd move/contracts/message-board && ./sh_scripts/fmt.sh`
- **Package name**: `message-board`
- **Contract address**: Stored in `contract_address.txt`

### Next.js Frontend (`/next-app/`)

- **Framework**: Next.js 14.2.3 with TypeScript
- **UI Library**: Radix UI components with Tailwind CSS
- **Key dependencies**: @aptos-labs/ts-sdk, @aptos-labs/wallet-adapter-react
- **Database**: Neon PostgreSQL
- **Commands**:
  - Dev: `npm run dev`
  - Build: `npm run build`
  - Lint: `npm run lint`
- **Key components**: Message board, wallet connection, analytics dashboard

### Node Scripts (`/node-script/`)

- **Purpose**: Quick scripts to interact with the contract
- **Commands**: `npm run <script-name>`
- **Available scripts**: create_message, get_message, update_message, yolo

### Rust Indexer (`/rust-indexer/`)

- **Framework**: Uses aptos-indexer-processor-sdk
- **Database**: PostgreSQL with Diesel ORM
- **Migrations**: Located in `src/db_migrations/migrations/`
- **Build command**: `cargo build`
- **Run command**: `cargo run`
- **Config files**: `config.yaml`, `local.config.yaml`, `cloud.config.yaml`

### TypeScript Indexer (`/ts-indexer/`)

- **Purpose**: Simpler alternative to Rust indexer for prototyping
- **Database**: SQLite (`indexer.db`)
- **Commands**: `npm start`

## Development Workflow

### Contract Development

1. Navigate to `move/contracts/message-board/`
2. Edit contract in `sources/`
3. Test with `./sh_scripts/test.sh`
4. Format with `./sh_scripts/fmt.sh`
5. Deploy with `./sh_scripts/deploy.sh`

### Frontend Development

1. Navigate to `next-app/`
2. Start dev server: `npm run dev`
3. Build for production: `npm run build`
4. Lint code: `npm run lint`

### Indexer Development

- **Rust**: Use `rust-indexer/` for production
- **TypeScript**: Use `ts-indexer/` for rapid prototyping

## Important Files

- **Contract ABI**: Generated in `next-app/src/lib/abi/message_board_abi.ts`
- **Database models**:
  - Frontend: `next-app/src/lib/type/`
  - Rust indexer: `rust-indexer/src/db_models/`
- **Utils**: `next-app/src/lib/utils.ts`

## Database Schema

Key tables:

- `messages`: Stores message board messages
- `user_stats`: User analytics and statistics
- `processor_status`: Indexer processing status
- `ledger_infos`: Blockchain ledger information

## Testing

- **Move contracts**: `cd move/contracts/message-board && npm test`
- **Frontend**: Uses Next.js built-in testing with `npm run lint`
- **Integration tests**: Available in `move/tests/`

## Configuration

- **Move.toml**: Contract configuration and dependencies
- **next.config.mjs**: Next.js configuration
- **Cargo.toml**: Rust indexer dependencies
- **Config files**: Various YAML configs for different environments

## GitHub Codespace Support

Pre-configured for GitHub Codespaces with all dependencies (Rust, Docker, Google Cloud CLI) ready. Install Aptos CLI with:

```bash
curl -fsSL "https://aptos.dev/scripts/install_cli.py" | python3
```

## Video Tutorials

- Contract and frontend: English and Chinese versions available
- Custom indexer: English and Chinese versions available
- GitHub Codespace development: English and Chinese versions available

## Recommended Development Flow

1. Start with contract development in `move/`
2. Generate ABIs and update frontend
3. Develop frontend features in `next-app/`
4. Set up indexer (TypeScript for prototyping, Rust for production)
5. Test end-to-end functionality

## Key Dependencies

- **Aptos SDK**: @aptos-labs/ts-sdk
- **Wallet Adapter**: @aptos-labs/wallet-adapter-react
- **UI Framework**: Radix UI + Tailwind CSS
- **State Management**: @tanstack/react-query
- **Database**: Neon PostgreSQL (frontend), PostgreSQL/SQLite (indexers)
