import { Message } from "@/components/Message";

export default function MessagePage({
  params,
}: {
  params: { messageObjectAddress: `0x${string}` };
}) {
  const { messageObjectAddress } = params;

  return <Message messageObjectAddress={messageObjectAddress} />;
}
