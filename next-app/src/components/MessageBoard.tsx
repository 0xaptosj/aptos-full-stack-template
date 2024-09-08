import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { DataTable } from "@/components/message-board/data-table";
import { columns } from "@/components/message-board/columns";

export const MessageBoard = async () => {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Message Board</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-wrap gap-4">
        <DataTable columns={columns} />
      </CardContent>
    </Card>
  );
};
