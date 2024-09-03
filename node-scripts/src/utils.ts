import "dotenv/config";
import { env } from "process";
import {
  Account,
  Aptos,
  AptosConfig,
  Ed25519PrivateKey,
  Network,
} from "@aptos-labs/ts-sdk";
import { createSurfClient } from "@thalalabs/surf";
import { ABI } from "./abis/message_board_abi";

export const getAptosClient = () => {
  return new Aptos(
    new AptosConfig({
      network: Network.TESTNET,
    })
  );
};

export const getSurfClient = () => {
  return createSurfClient(getAptosClient()).useABI(ABI);
};

export const getAccount = () => {
  if (!env.PRIVATE_KEY && env.PRIVATE_KEY === "to_fill") {
    throw new Error("Please fill in your private key");
  }

  return Account.fromPrivateKey({
    privateKey: new Ed25519PrivateKey(env.PRIVATE_KEY!),
  });
};
