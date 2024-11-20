"use server";

import { getLastSuccessVersion } from "@/db/getLastSuccessVersion";
import { GetMessageProps, getMessage } from "@/db/getMessage";
import { GetMessagesProps, getMessages } from "@/db/getMessages";
import { getUserStats, GetUserStatsProps } from "@/db/getUserStats";
import { getAptosClient } from "@/lib/aptos";
import { Message } from "@/lib/type/message";
import { UserStat } from "@/lib/type/user_stats";
import {
  Account,
  AccountAuthenticator,
  Deserializer,
  Ed25519PrivateKey,
  SimpleTransaction,
} from "@aptos-labs/ts-sdk";

export const getMessagesOnServer = async ({
  page,
  limit,
  sortedBy,
  order,
}: GetMessagesProps): Promise<{
  messages: Message[];
  total: number;
}> => {
  return getMessages({ page, limit, sortedBy, order });
};

export const getMessageOnServer = async ({
  messageObjAddr,
}: GetMessageProps): Promise<{
  message: Message;
}> => {
  return getMessage({ messageObjAddr });
};

export const getLastVersionOnServer = async (): Promise<number> => {
  return getLastSuccessVersion();
};

export const getUserStatsOnServer = async ({
  page,
  limit,
  sortedBy,
  order,
}: GetUserStatsProps): Promise<{
  userStats: UserStat[];
  total: number;
}> => {
  return getUserStats({ page, limit, sortedBy, order });
};

type SponsorTxOnServerProps = {
  transactionBytes: number[]; // representing Unit8Array
  senderAuthenticatorBytes: number[]; // representing Unit8Array
};
export const sponsorTxOnServer = async ({
  transactionBytes,
  senderAuthenticatorBytes,
}: SponsorTxOnServerProps) => {
  const transaction = SimpleTransaction.deserialize(
    new Deserializer(new Uint8Array(transactionBytes))
  );
  const senderAuthenticator = AccountAuthenticator.deserialize(
    new Deserializer(new Uint8Array(senderAuthenticatorBytes))
  );

  const sponsor = Account.fromPrivateKey({
    privateKey: new Ed25519PrivateKey(process.env.TX_SPONSOR_PRIVATE_KEY!),
  });

  const feePayerAuthenticator = getAptosClient().transaction.signAsFeePayer({
    signer: sponsor,
    transaction,
  });

  return await getAptosClient().transaction.submit.simple({
    transaction,
    senderAuthenticator,
    feePayerAuthenticator,
  });
};
