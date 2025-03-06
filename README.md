# A template to build a full stack app on Aptos

- [Video tutorial in English](https://www.youtube.com/watch?v=-UkbHdeSImc)
- [中文视频教程](https://www.youtube.com/watch?v=uAfK1Lpr33M)

This template is an opinionated alternative template to [CAD (create-aptos-dapp)](https://aptos.dev/en/build/create-aptos-dapp).

Please read each directory's README carefully to understand how to use the template.

- `indexer` directory for custom indexer on the contract
- `next-app` directory for the Next.js frontend
- `move` directory for the contract and integration tests
- `node-scripts` directory for some quick scripts to interact with the contract in Node.js

## Using [GitHub Codespace](https://github.com/features/codespaces)

- [Video tutorial in English](https://www.youtube.com/watch?v=RJnlSwyNI8Q)
- [中文视频教程](https://www.youtube.com/watch?v=kAM0zH6N6pc)

If you are on a Windows machine or have trouble pulling a docker image or connecting to RPC, you can use GitHub codespace as a remote development server, this repo is pre-configured for codespace with all the dependencies (Rust, docker, google cloud cli) ready. If you use codespace, you can install aptos-cli via 

```sh
curl -fsSL "https://aptos.dev/scripts/install_cli.py" | python3
```

When you see warning like codespace is running low on disk space, you can prune docker cache 

```sh
docker system prune -a

docker builder prune

docker system df
```
