"use server";

import { sql } from "@vercel/postgres";

import { MessageBoardColumns } from "@/lib/type/message";

type Props = {
  page: number;
  limit: number;
  sortedBy: "id" | "creation_timestamp";
  order: "ASC" | "DESC";
};

export const getMessages = async ({
  page,
  limit,
  sortedBy,
  order,
}: Props): Promise<{
  messages: MessageBoardColumns[];
  totalMessages: number;
}> => {
  const { rows } = await sql`
        SELECT id, creation_timestamp FROM messages
        ORDER BY ${sortedBy} ${order}
        LIMIT ${limit} OFFSET ${(page - 1) * limit};
    `;
  const messages = rows.map((row) => {
    return {
      id: row.id,
      creation_timestamp: new Date(row.creation_timestamp),
    };
  });
  const { rows: count } = await sql`
        SELECT COUNT(*) FROM messages;
    `;
  return { messages, totalMessages: count[0].count };
};
