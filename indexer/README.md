# Overview

This indexer is created from indexer-sdk, see a more detailed readme in [example indexer repo](https://github.com/aptos-labs/aptos-indexer-processor-example).

We use the term indexer and processor interchangeably.

# Running the indexer

**Note: all commends below need to be run in the current indexer directory instead of root directory.**

## Steps

Drop the DB if exists. You cannot do this if you are using a cloud provider. Follow the revert migration command below instead.

```sh
psql postgres://username@127.0.0.1:5432/postgres \
    -c 'DROP DATABASE IF EXISTS "example-indexer"'
```

Create the DB.

```sh
psql postgres://username@127.0.0.1:5432/postgres \
    -c 'CREATE DATABASE "example-indexer"'
```

Create a new migration file.

```sh
diesel migration generate create-abc-table \
    --config-file="src/db/postgres/diesel.toml"
```

Run all pending migrations.

```sh
diesel migration run \
    --database-url="postgresql://username:@localhost:5432/example-indexer" \
    --config-file="src/db/postgres/diesel.toml"
```

In case you want to revert all migrations. On cloud provider, you cannot drop database, so you need to revert all migrations if you want to reset.

```sh
diesel migration revert \
	--all \
    --database-url="postgresql://username:@localhost:5432/example-indexer" \
	--config-file="src/db/postgres/diesel.toml"
```

Create a `config.yaml` file from `example.config.yaml` file to point to the correct network, db url, start version, etc. Run the indexer.

```sh
cargo run --release -- -c config.yaml
```

You should see the indexer start to index Aptos blockchain events!

```sh
"timestamp":"2024-08-15T01:06:35.169217Z","level":"INFO","message":"[Transaction Stream] Received transactions from GRPC.","stream_address":"https://grpc.testnet.aptoslabs.com/","connection_id":"5575cb8c-61fb-498f-aaae-868d1e8773ac","start_version":0,"end_version":4999,"start_txn_timestamp_iso":"1970-01-01T00:00:00.000000000Z","end_txn_timestamp_iso":"2022-09-09T01:49:02.023089000Z","num_of_transactions":5000,"size_in_bytes":5708539,"duration_in_secs":0.310734,"tps":16078,"bytes_per_sec":18371143.80788713,"filename":"/Users/reneetso/.cargo/git/checkouts/aptos-indexer-processor-sdk-2f3940a333c8389d/e1e1bdd/rust/transaction-stream/src/transaction_stream.rs","line_number":400,"threadName":"tokio-runtime-worker","threadId":"ThreadId(6)"
"timestamp":"2024-08-15T01:06:35.257756Z","level":"INFO","message":"Events version [0, 4999] stored successfully","filename":"src/processors/events/events_storer.rs","line_number":75,"threadName":"tokio-runtime-worker","threadId":"ThreadId(10)"
"timestamp":"2024-08-15T01:06:35.257801Z","level":"INFO","message":"Finished processing events from versions [0, 4999]","filename":"src/processors/events/events_processor.rs","line_number":90,"threadName":"tokio-runtime-worker","threadId":"ThreadId(17)"
```

# Running the indexer as a docker container for cloud deployment

I'm using GCP Cloud Run and Artifact Registry.

You can learn more about publishing to Artifact Registry on their docs:

- https://cloud.google.com/artifact-registry/docs/docker/pushing-and-pulling#pushing
- https://cloud.google.com/artifact-registry/docs/docker/store-docker-container-images

And deploying to Cloud Run:

- https://cloud.google.com/run/docs/quickstarts/deploy-container

## Build the docker image and run the container locally

Build the docker image.

```sh
docker build -t indexer .
```

Run the docker container.

```sh
# docker run -v $(pwd):/usr/src/app -p 8080:8080 -it indexer
docker run -p 8080:8080 -it indexer
```

## Push the docker image to Artifact Registry

Tag the docker image.

```sh
docker tag indexer us-west2-docker.pkg.dev/indexer-sdk-demo/indexer-sdk-demo/indexer
```

Login to google cloud

```sh
gcloud auth login
```

Push the docker image to the container registry.

```sh
docker push us-west2-docker.pkg.dev/indexer-sdk-demo/indexer-sdk-demo/indexer
```



gcloud config set project indexer-sdk-demo
<!-- gcloud builds submit . -->
gcloud builds submit --tag gcr.io/indexer-sdk-demo/indexer


gcloud run deploy $indexer \
    --image gcr.io/indexer-sdk-demo/indexer:latest \
    --region us-west2 --platform managed \
    --allow-unauthenticated