import { sql } from "@vercel/postgres";

import { MessageInUi, MessageInDb } from "@/lib/type/message";

export type GetMessageProps = {
  messageObjAddr: `0x${string}`;
};

export const getMessage = async ({
  messageObjAddr,
}: GetMessageProps): Promise<{
  message: MessageInUi;
}> => {
  const query = `SELECT * FROM messages WHERE message_obj_addr = $1`;
  const { rows } = await sql.query(query, [messageObjAddr]);
  if (rows.length === 0) {
    throw new Error("Message not found");
  }
  const message: MessageInDb = rows[0];
  const messageConverted = {
    message_obj_addr: message.message_obj_addr as `0x${string}`,
    creator_addr: message.creator_addr as `0x${string}`,
    creation_timestamp: new Date(
      message.creation_timestamp * 1000
    ).toLocaleString(),
    last_update_timestamp: new Date(
      message.last_update_timestamp * 1000
    ).toLocaleString(),
    content: message.content,
  };
  return { message: messageConverted };
};
