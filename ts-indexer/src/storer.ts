import Database from "better-sqlite3";
import type { Database as DatabaseType } from "better-sqlite3";
import { streamTransactions } from "./streamTx";
import "dotenv/config";

async function* streamAndPersistTransactions({
  db,
  ...opts
}: Omit<Parameters<typeof streamTransactions>[number], "startingVersion"> & {
  db: DatabaseType;
}) {
  db.exec(`
CREATE TABLE IF NOT EXISTS processor_status(
    processor TEXT NOT NULL,
    last_success_version INTEGER NOT NULL,
    last_transaction_timestamp TIMESTAMP NOT NULL,
    last_updated TIMESTAMP NOT NULL,
    PRIMARY KEY (processor)
)
`);

  const lastSuccessVersion = db
    .prepare(
      `
SELECT last_success_version FROM processor_status WHERE processor = 'my_processor'
      `
    )
    .get() as { last_success_version: number } | undefined;

  const startingVersion = lastSuccessVersion
    ? lastSuccessVersion.last_success_version + 1
    : process.env.STARTING_VERSION!;

  for await (const event of streamTransactions({
    ...opts,
    startingVersion: BigInt(startingVersion),
  })) {
    yield event;

    if (event.type === "data") {
      const lastSuccessTx = event.transactions[event.transactions.length - 1];

      db.prepare(
        `
INSERT INTO processor_status(processor, last_success_version, last_transaction_timestamp, last_updated)
VALUES(?, ?, ?, ?)
ON CONFLICT(processor)
DO UPDATE SET
    last_success_version = excluded.last_success_version,
    last_transaction_timestamp = excluded.last_transaction_timestamp,
    last_updated = excluded.last_updated
        `
      ).run(
        "my_processor",
        lastSuccessTx.version,
        lastSuccessTx.timestamp?.seconds,
        Math.floor(Date.now() / 1000)
      );
    }
  }
}

const db = new Database("indexer.db");

db.pragma("journal_mode = WAL");

const run = async () => {
  for await (const event of streamAndPersistTransactions({
    db,
    url: process.env.INDEXER_GRPC_URL!,
    apiKey: process.env.APTOS_API_KEY!,
  })) {
    switch (event.type) {
      case "data": {
        const startVersion = event.transactions[0].version!;
        const endVersion =
          event.transactions[event.transactions.length - 1].version!;

        console.debug(
          `Got ${event.transactions.length} transaction(s) from version ${startVersion} to ${endVersion}.`
        );
        break;
      }
      case "error": {
        console.error(event.error);
        break;
      }
      case "metadata": {
        console.log(event.metadata);
        break;
      }
      case "status": {
        console.log(event.status);
        break;
      }
    }
  }
};

run();
