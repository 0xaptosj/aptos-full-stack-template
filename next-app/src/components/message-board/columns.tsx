"use client";

import { ColumnDef } from "@tanstack/react-table";

import { DataTableColumnHeader } from "@/components/message-board/data-table-column-header";
import { DataTableRowActions } from "@/components/message-board/data-table-row-actions";
import { MessageBoardColumns } from "@/lib/type/message";

export const columns: ColumnDef<MessageBoardColumns>[] = [
  {
    accessorKey: "id",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Message ID" />
    ),
    cell: ({ row }) => <div className="w-[80px]">{row.getValue("id")}</div>,
    enableSorting: true,
  },
  {
    accessorKey: "creation_timestamp",
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Creation Timestamp" />
    ),
    cell: ({ row }) => (
      <div className="w-[80px]">{row.getValue("creation_timestamp")}</div>
    ),
    enableSorting: true,
  },
  {
    id: "actions",
    cell: ({ row }) => <DataTableRowActions row={row} />,
  },
];
