"use server";

import { revalidatePath } from "next/cache";

import { GetMessageProps, getMessage } from "@/db/getMessage";
import { GetMessagesProps, getMessages } from "@/db/getMessages";
import { MessageBoardColumns, MessageInUi } from "@/lib/type/message";

export async function revalidateHome() {
  revalidatePath("/");
}

export const getMessagesOnServer = async ({
  page,
  limit,
  sortedBy,
  order,
}: GetMessagesProps): Promise<{
  messages: MessageBoardColumns[];
  totalMessages: number;
}> => {
  return getMessages({ page, limit, sortedBy, order });
};

export const getMessageOnServer = async ({
  messageId,
}: GetMessageProps): Promise<{
  message: MessageInUi;
}> => {
  return getMessage({ messageId });
};
