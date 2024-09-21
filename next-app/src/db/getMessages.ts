import { getPostgresClient } from "@/lib/db";
import { MessageBoardColumns } from "@/lib/type/message";

export type GetMessagesProps = {
  page: number;
  limit: number;
  sortedBy: "creation_timestamp";
  order: "ASC" | "DESC";
};

export const getMessages = async ({
  page,
  limit,
  sortedBy,
  order,
}: GetMessagesProps): Promise<{
  messages: MessageBoardColumns[];
  totalMessages: number;
}> => {
  const rows = await getPostgresClient()(
    `SELECT message_obj_addr, creation_timestamp, content FROM messages ORDER BY ${sortedBy} ${order} LIMIT ${limit} OFFSET ${
      (page - 1) * limit
    }`
  );

  const messages = rows.map((row) => {
    return {
      message_obj_addr: row.message_obj_addr,
      creation_timestamp: row.creation_timestamp,
      content: row.content,
    };
  });

  const rows2 = await getPostgresClient()(`
        SELECT COUNT(*) FROM messages;
    `);
  const count = rows2[0].count;

  return { messages, totalMessages: count };
};
