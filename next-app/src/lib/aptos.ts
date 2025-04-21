import { Aptos, AptosConfig, Network } from "@aptos-labs/ts-sdk";
import { createSurfClient } from "@thalalabs/surf";

import { ABI } from "@/lib/abi/message_board_abi";

export const NETWORK = process.env.NEXT_PUBLIC_NETWORK! as Network;

export const getAptosClient = (api_key?: string) =>
  new Aptos(
    new AptosConfig({
      network: NETWORK,
      clientConfig: {
        API_KEY: api_key,
      },
    })
  );

export const getSurfClient = () =>
  createSurfClient(getAptosClient()).useABI(ABI);
