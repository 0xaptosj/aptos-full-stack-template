#!/bin/sh

set -e

echo "##### Lint and format #####"

aptos move fmt

aptos move lint \
  --named-addresses message_board_addr=0x10000 # dummy address
