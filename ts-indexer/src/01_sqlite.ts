import Database from "better-sqlite3";
import type { Database as DatabaseType } from "better-sqlite3";
import { streamTransactions } from "./streamTx";

async function* streamAndPersistTransactions({
  db,
  ...opts
}: Omit<Parameters<typeof streamTransactions>[number], "startingVersion"> & {
  db: DatabaseType;
}) {
  db.exec(`CREATE TABLE IF NOT EXISTS kv(k PRIMARY KEY, v)`);

  const startingVersionRow = db
    .prepare(`SELECT v FROM kv WHERE k = 'startingVersion'`)
    .get() as { v: string | number | bigint } | undefined;

  const startingVersion = startingVersionRow?.v ?? 1948140715n;

  for await (const event of streamTransactions({
    ...opts,
    startingVersion: BigInt(startingVersion),
  })) {
    yield event;

    if (event.type === "data") {
      const nextStartingVersion =
        event.transactions[event.transactions.length - 1].version! + 1n;

      db.prepare(
        `INSERT INTO kv(k, v) VALUES('startingVersion', ?) ON CONFLICT(k) DO UPDATE SET v = excluded.v`
      ).run(nextStartingVersion);
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
        if (event.chainId !== 1n) {
          throw new Error(
            `Transaction stream returned a chainId of ${event.chainId}, but expected mainnet chainId=1`
          );
        }

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
