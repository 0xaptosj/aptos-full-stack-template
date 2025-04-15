import type { Database as DatabaseType } from "better-sqlite3";
import { Transaction } from "@aptos-labs/aptos-protos/dist/esm/index.aptos.transaction.v1";
import { standardizeAddress } from "./utils";

export const extractAndSaveEvents = async (
  db: DatabaseType,
  transaction: Transaction
) => {
  if (!transaction.user || !transaction.user.events) {
    return;
  }

  for (let i = 0; i < transaction.user.events.length; i++) {
    const txEvent = transaction.user.events[i];
    const eventType = (txEvent.typeStr || "").split("::");
    if (eventType.length < 3) {
      continue;
    }
    const eventTypeAddr = standardizeAddress(eventType[0]);
    if (
      `${eventTypeAddr}::${eventType[1]}::${eventType[2]}` ===
        `${process.env.CONTRACT_ADDRESS}::message_board::CreateMessageEvent` ||
      `${eventTypeAddr}::${eventType[1]}::${eventType[2]}` ===
        `${process.env.CONTRACT_ADDRESS}::message_board::UpdateMessageEvent`
    ) {
      if (!txEvent.data) {
        console.error(`Event has no data: ${JSON.stringify(txEvent)}`);
        continue;
      }
      const eventData: {
        message_obj_addr: string;
        message: {
          creator: string;
          content: string;
          creation_timestamp: number;
          last_update_timestamp: number;
        };
      } = JSON.parse(txEvent.data);
      console.debug(`Event data: ${JSON.stringify(eventData)}`);

      const messageObjAddr = standardizeAddress(eventData.message_obj_addr);
      const creatorAddr = standardizeAddress(eventData.message.creator);
      const creationTimestamp = eventData.message.creation_timestamp;
      const lastUpdateTimestamp = eventData.message.last_update_timestamp;
      const content = eventData.message.content;

      db.prepare(
        `
    INSERT INTO messages(
        message_obj_addr,
        creator_addr,
        creation_timestamp,
        last_update_timestamp,
        last_update_event_idx,
        content
    ) VALUES(?, ?, ?, ?, ?, ?)
    ON CONFLICT(message_obj_addr)
    DO UPDATE SET
        creator_addr = excluded.creator_addr,
        creation_timestamp = excluded.creation_timestamp,
        last_update_timestamp = excluded.last_update_timestamp,
        last_update_event_idx = excluded.last_update_event_idx,
        content = excluded.content
    WHERE excluded.last_update_event_idx > messages.last_update_event_idx
            `
      ).run(
        messageObjAddr,
        creatorAddr,
        creationTimestamp,
        lastUpdateTimestamp,
        eventType[2] === "CreateMessageEvent" ? 0 : i,
        content
      );
    }
  }
};
