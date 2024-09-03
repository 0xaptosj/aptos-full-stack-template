import { createSurfClient } from "@thalalabs/surf";
import { Aptos, AptosConfig, Network } from "@aptos-labs/ts-sdk";
import { ABI } from "@/abi/message_board_abi";

export const NETWORK_NAME = Network.TESTNET;

export const aptosClient = () => {
  return TESTNET_CLIENT;
};

export const surfClient = () => {
  return createSurfClient(aptosClient()).useABI(ABI);
};

// Testnet client
export const TESTNET_CONFIG = new AptosConfig({ network: Network.TESTNET });
export const TESTNET_CLIENT = new Aptos(TESTNET_CONFIG);
