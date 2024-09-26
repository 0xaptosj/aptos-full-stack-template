import { getPostgresClient } from "@/lib/db";
import { Message } from "@/lib/type/message";

export type GetMessageProps = {
  messageObjAddr: `0x${string}`;
};

export const getMessage = async ({
  messageObjAddr,
}: GetMessageProps): Promise<{
  message: Message;
}> => {
  const rows = await getPostgresClient()(
    `SELECT * FROM messages WHERE message_obj_addr = '${messageObjAddr}'`
  );
  if (rows.length === 0) {
    throw new Error("Message not found");
  }
  const message = rows[0] as Message;
  return { message };
};
