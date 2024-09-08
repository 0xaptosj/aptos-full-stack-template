"use server";

import { FieldDef, sql } from "@vercel/postgres";

import { Message } from "@/lib/type/Message";

type Props = {
  limit: number;
  offset: number;
  sortedBy: string;
  order: "ASC" | "DESC";
};

export const getMessages = async ({
  limit,
  offset,
  sortedBy,
  order,
}: Props): Promise<{ rows: Message[]; fields: FieldDef[] }> => {
  const { rows, fields, rowCount } = await sql`
        SELECT * FROM messages
        ORDER BY ${sortedBy} ${order}
        LIMIT ${limit} OFFSET ${offset};
    `;
  return { rows: rows as Message[], fields };
};
