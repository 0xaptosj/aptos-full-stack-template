import { sql } from "@vercel/postgres";

import { MessageInUi, MessageInDb } from "@/lib/type/message";

export type GetMessageProps = {
  messageId: number;
};

export const getMessage = async ({
  messageId,
}: GetMessageProps): Promise<{
  message: MessageInUi;
}> => {
  const query = `SELECT * FROM messages WHERE id = $1`;
  console.log("query", query);
  const { rows } = await sql.query(query, [messageId]);
  if (rows.length === 0) {
    throw new Error("Message not found");
  }
  const message: MessageInDb = rows[0];
  const messageConverted = {
    id: message.id,
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
