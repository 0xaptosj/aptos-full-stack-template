#!/bin/sh

set -e

echo "##### Running tests #####"

aptos move test \
  --dev
