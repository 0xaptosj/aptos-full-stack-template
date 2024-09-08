import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { DataTable } from "./message-board/data-table";
import { columns } from "./message-board/columns";
import { sql } from "@vercel/postgres";

const pageSize = 10; // Number of records per page
const pageNumber = 1; // Specific page number

const offset = (pageNumber - 1) * pageSize;

export const MessageBoard = async () => {
  // const messageObjectAddresses = await surfClient()
  //   .view.get_message_objects({
  //     typeArguments: [],
  //     functionArguments: [undefined, undefined],
  //   })
  //   .then((res) => {
  //     return res[0].map((obj) => {
  //       return {
  //         messageObjectAddress: obj.inner,
  //       };
  //     });
  //   });
  // Fetching records for the specific page number returning 10 records per page
  const { rows, fields } =
    await sql`SELECT * FROM messages LIMIT ${pageSize} OFFSET ${offset};`;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Message Board</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-wrap gap-4">
        {/* <DataTable columns={columns} data={messageObjectAddresses} /> */}
        <DataTable columns={columns} />
      </CardContent>
    </Card>
  );
};
