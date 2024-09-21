import { getPostgresClient } from "@/lib/db";
import { MessageOnUi, MessageInDb } from "@/lib/type/message";

export type GetMessageProps = {
  messageObjAddr: `0x${string}`;
};

export const getMessage = async ({
  messageObjAddr,
}: GetMessageProps): Promise<{
  message: MessageOnUi;
}> => {
  const rows =
    await getPostgresClient()`SELECT * FROM messages WHERE message_obj_addr = ${messageObjAddr}`;
  if (rows.length === 0) {
    throw new Error("Message not found");
  }
  const message = rows[0] as MessageInDb;
  const messageConverted = {
    message_obj_addr: message.message_obj_addr as `0x${string}`,
    creator_addr: message.creator_addr as `0x${string}`,
    creation_timestamp: message.creation_timestamp,
    last_update_timestamp: message.last_update_timestamp,
    content: message.content,
  };
  return { message: messageConverted };
};
