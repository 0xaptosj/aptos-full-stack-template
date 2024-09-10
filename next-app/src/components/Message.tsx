import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { LabelValueGrid } from "@/components/LabelValueGrid";
import { getMessage } from "@/db/getMessage";
import { NETWORK } from "@/lib/aptos";

interface MessageProps {
  messageId: number;
}

export async function Message({ messageId }: MessageProps) {
  const { message } = await getMessage({ messageId });

  return (
    <Card>
      <CardHeader>
        <CardTitle>Message</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-10 pt-6">
        <div className="flex flex-col gap-6">
          <LabelValueGrid
            items={[
              {
                label: "ID",
                value: <p>{message.id}</p>,
              },
              {
                label: "Creator address",
                value: (
                  <p>
                    <a
                      href={`https://explorer.aptoslabs.com/account/${message.creator_addr}?network=${NETWORK}`}
                      target="_blank"
                      rel="noreferrer"
                      className="text-blue-600 dark:text-blue-300"
                    >
                      {message.creator_addr}
                    </a>
                  </p>
                ),
              },
              {
                label: "Message object address",
                value: (
                  <p>
                    <a
                      href={`https://explorer.aptoslabs.com/object/${message.message_obj_addr}?network=${NETWORK}`}
                      target="_blank"
                      rel="noreferrer"
                      className="text-blue-600 dark:text-blue-300"
                    >
                      {message.message_obj_addr}
                    </a>
                  </p>
                ),
              },
              {
                label: "Creation timestamp",
                value: <p>{message.creation_timestamp}</p>,
              },
              {
                label: "Last update timestamp",
                value: <p>{message.last_update_timestamp}</p>,
              },
              {
                label: "Content",
                value: <p>{message.content}</p>,
              },
            ]}
          />
        </div>
      </CardContent>
    </Card>
  );
}
