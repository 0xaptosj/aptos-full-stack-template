export type Message = {
  message_obj_addr: `0x${string}`;
  creator_addr: `0x${string}`;
  creation_timestamp: number;
  last_update_timestamp: number;
  last_update_event_idx: number;
  content: string;
};
