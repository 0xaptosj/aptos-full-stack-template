import { Message } from "@/components/Message";

export default function MessagePage({
  params,
}: {
  params: { messageId: number };
}) {
  const { messageId } = params;

  return <Message messageId={messageId} />;
}
