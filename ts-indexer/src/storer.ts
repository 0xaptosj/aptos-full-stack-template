import "dotenv/config";
import Database from "better-sqlite3";
import type { Database as DatabaseType } from "better-sqlite3";
import { streamTransactions } from "./streamTx";
import { extractAndSaveEvents } from "./extractor";

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

  db.exec(`
CREATE TABLE IF NOT EXISTS messages(
    message_obj_addr TEXT NOT NULL,
    creator_addr TEXT NOT NULL,
    creation_timestamp INTEGER NOT NULL,
    last_update_timestamp INTEGER NOT NULL,
    -- we store the event index so when we update in batch,
    -- we ignore when the event index is less than the last update event index
    last_update_event_idx INTEGER NOT NULL,
    content TEXT NOT NULL,
    PRIMARY KEY (message_obj_addr)
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

  for await (const streamOutput of streamTransactions({
    ...opts,
    startingVersion: BigInt(startingVersion),
  })) {
    yield streamOutput;

    if (streamOutput.type === "data") {
      for (const transaction of streamOutput.transactions) {
        extractAndSaveEvents(db, transaction);
      }

      const lastSuccessTx =
        streamOutput.transactions[streamOutput.transactions.length - 1];

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

const run = async () => {
  const db = new Database("indexer.db");
  db.pragma("journal_mode = WAL");

  for await (const streamOutput of streamAndPersistTransactions({
    db,
    url: process.env.INDEXER_GRPC_URL!,
    apiKey: process.env.APTOS_API_KEY!,
  })) {
    switch (streamOutput.type) {
      case "data": {
        const startVersion = streamOutput.transactions[0].version!;
        const endVersion =
          streamOutput.transactions[streamOutput.transactions.length - 1]
            .version!;

        console.debug(
          `Got ${streamOutput.transactions.length} transaction(s) from version ${startVersion} to ${endVersion}.`
        );
        break;
      }
      case "error": {
        console.error(streamOutput.error);
        break;
      }
      case "metadata": {
        console.log(streamOutput.metadata);
        break;
      }
      case "status": {
        console.log(streamOutput.status);
        break;
      }
    }
  }
};

run();
