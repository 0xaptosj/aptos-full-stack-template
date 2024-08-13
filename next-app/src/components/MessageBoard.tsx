import { surfClient } from "@/lib/aptos";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { DataTable } from "./message-board/data-table";
import { columns } from "./message-board/columns";

export const MessageBoard = async () => {
  const messageObjectAddresses = await surfClient()
    .view.get_message_objects({
      typeArguments: [],
      functionArguments: [undefined, undefined],
    })
    .then((res) => {
      return res[0].map((obj) => {
        return {
          messageObjectAddress: obj.inner,
        };
      });
    });

  return (
    <Card>
      <CardHeader>
        <CardTitle>Message Board</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-wrap gap-4">
        <DataTable columns={columns} data={messageObjectAddresses} />
      </CardContent>
    </Card>
  );
};
