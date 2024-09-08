"use client";

import { useWallet } from "@aptos-labs/wallet-adapter-react";

import { WalletSelection } from "@/components/wallet/WalletSelection";
import { WalletConnection } from "@/components/wallet/WalletConnection";

export const Wallet = () => {
  const { account, connected, network, wallet } = useWallet();

  return (
    <>
      <WalletSelection />
      {connected && (
        <WalletConnection account={account} network={network} wallet={wallet} />
      )}
    </>
  );
};
