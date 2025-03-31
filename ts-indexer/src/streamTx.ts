import { aptos } from "@aptos-labs/aptos-protos";
import { ChannelCredentials, Metadata, type StatusObject } from "@grpc/grpc-js";

export async function* streamTransactions(opts: {
  url: string;
  apiKey: string;
  startingVersion: bigint;
}) {
  const client = new aptos.indexer.v1.RawDataClient(
    opts.url,
    ChannelCredentials.createSsl(),
    {
      "grpc.keepalive_time_ms": 1000,
      // 0 - No compression
      // 1 - Compress with DEFLATE algorithm
      // 2 - Compress with GZIP algorithm
      // 3 - Stream compression with GZIP algorithm
      "grpc.default_compression_algorithm": 2,
      // 0 - No compression
      // 1 - Low compression level
      // 2 - Medium compression level
      // 3 - High compression level
      "grpc.default_compression_level": 3,
      // -1 means unlimited
      "grpc.max_receive_message_length": -1,
      // -1 means unlimited
      "grpc.max_send_message_length": -1,
    }
  );

  const metadata = new Metadata();
  metadata.set("Authorization", `Bearer ${opts.apiKey}`);

  const request: aptos.indexer.v1.GetTransactionsRequest = {
    startingVersion: opts.startingVersion,
  };

  const stream = client.getTransactions(request, metadata);

  const output = new ReadableStream<
    | {
        type: "data";
        chainId: bigint;
        transactions: aptos.transaction.v1.Transaction[];
      }
    | { type: "error"; error: Error }
    | { type: "metadata"; metadata: Metadata }
    | { type: "status"; status: StatusObject }
  >({
    start(controller) {
      stream.on("data", (response: aptos.indexer.v1.TransactionsResponse) => {
        const chainId = response.chainId;
        if (chainId === undefined) {
          return;
        }

        const transactions = response.transactions;
        if (transactions === undefined || transactions.length === 0) {
          return;
        }

        controller.enqueue({ type: "data" as const, chainId, transactions });
      });

      stream.on("error", (error) => {
        controller.enqueue({ type: "error" as const, error });
      });

      stream.on("metadata", (metadata) => {
        controller.enqueue({ type: "metadata" as const, metadata });
      });

      stream.on("status", (status) => {
        controller.enqueue({ type: "status" as const, status });
      });

      stream.on("end", () => {
        controller.close();
      });
    },
  });

  for await (const chunk of output) {
    yield chunk;
  }
}
