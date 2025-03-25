# aptos-indexer

ported over from https://github.com/lithdew/aptos-indexer

known issue, indexer stopped working at tx version `1608183999` which is aound 08/2024. I guess there was some breaking change introduced, and the aptos-proto js library hasn't been updated for a year. will see if error is gone after upgrading proto