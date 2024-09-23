import { MessageBoard } from "@/components/MessageBoard";
import { CreateMessage } from "@/components/CreateMessage";

export default function HomePage() {
  return (
    <>
      <MessageBoard />
      <CreateMessage />
    </>
  );
}
