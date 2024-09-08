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
  const query = `SELECT id, creation_timestamp FROM messages ORDER BY $1 LIMIT $2 OFFSET $3`;
  console.log("query", query);
  const { rows } = await sql.query(query, [
    // vercel has weird error that we cannot use `${sortedBy} ${order}` directly
    `${sortedBy} ${order}`,
    limit,
    (page - 1) * limit,
  ]);
  const messages = rows.map((row) => {
    return {
      id: row.id,
      creation_timestamp: new Date(
        row.creation_timestamp * 1000
      ).toLocaleString(),
    };
  });
  const { rows: count } = await sql`
        SELECT COUNT(*) FROM messages;
    `;
  return { messages, totalMessages: count[0].count };
};
