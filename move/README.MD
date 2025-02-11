# Move Contracts

This directory contains the contracts and integration tests. If you want to add a new contract, just create a new folder under `contracts` with the contract name and a `Move.toml` file inside it.

You probably want to learn the concept of module and package on Move. But TL;DR, a module is a single `.move` file similar to a `.sol` file, and a package is a collection of modules. Each package has a `Move.toml` file that describes the package and its dependencies.

https://aptos.dev/en/build/smart-contracts/book/packages

To run the integration test

```sh
npx aptos-workspace test
```

You can learn more about the integration test framework `aptos-workspace` on GitHub.

https://github.com/aptos-labs/workspace
